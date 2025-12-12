use crate::{
    ArcTopicDataMap, ArcTopicDataValue, DataPathSegment, ParsedDataPath, PipelineKernelErrorCode,
    PlainDataPath, VariablePredefineFunctionCaller,
};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_model::{StdErrorCode, StdR};

pub trait DataVisitorBase {
    /// returns empty vec when first segment identify the value is a vec type
    fn transform_none_value_for_first_segment(
        &self,
        data_path: &ParsedDataPath,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        if data_path.segments.is_empty() {
            // never happen, at least segments has one element
            // anyway, return none
            Ok(Arc::new(ArcTopicDataValue::None))
        } else {
            match &data_path.segments[0] {
                DataPathSegment::Plain(first_segment) => {
                    if first_segment.is_vec.unwrap_or(false) {
                        Ok(Arc::new(ArcTopicDataValue::Vec(vec![].into())))
                    } else {
                        Ok(Arc::new(ArcTopicDataValue::None))
                    }
                }
                DataPathSegment::Func(_) => Ok(Arc::new(ArcTopicDataValue::None)),
            }
        }
    }

    fn value_of_simple_path(&self, parsed_path: &ParsedDataPath) -> StdR<Arc<ArcTopicDataValue>>;

    fn value_of_plain_segment(
        &self,
        data: &Arc<ArcTopicDataValue>,
        segment: &PlainDataPath,
        full_path: &String,
    ) -> StdR<Arc<ArcTopicDataValue>>;

    fn value_of_complex_path(&self, parsed_path: &ParsedDataPath) -> StdR<Arc<ArcTopicDataValue>>;
}

impl DataVisitorBase for ArcTopicDataMap {
    /// simple path has only one segment
    fn value_of_simple_path(&self, path: &ParsedDataPath) -> StdR<Arc<ArcTopicDataValue>> {
        // use none if name not exists, never mind the array or not
        let value = self.get(&path.path).clone();
        if value.is_some() {
            Ok(value.unwrap().clone())
        } else {
            self.transform_none_value_for_first_segment(path)
        }
    }

    /// get value from given data by given segment
    /// only map and vec are supported
    /// - when given data is a map, return none when nothing found from this map,
    /// - when given data is a vec, then only none and map element are supported,
    ///   and the returned data is a vec.
    ///   - when given segment is identified as a vec,
    ///     - ignore the none element of given vec,
    ///     - ignore when nothing found from the map element of given vec,
    ///     - ignore the none value found from the map element of given vec,
    ///
    /// e.g.
    /// 1. given data is a map, as `{a: 1, b: 2, d: none}`,
    ///    - segment is `a` -> `1`
    ///    - segment is `c` -> `none`
    ///    - segment is `d` -> `none`
    /// 2. given data is a vec, as `[none, {a: [1, 2], b: 3, d: none}]`,
    ///    - segment is `a`, [is_vec] is not true -> `[none, 1, 2]`,
    ///    - segment is `a`, [is_vec] is true -> `[1, 2]`,
    ///    - segment is `c`, [is_vec] is not true -> `[none, none]`
    ///    - segment is `c`, [is_vec] is true -> `[]`
    ///    - segment is `d`, [is_vec] is not true -> `[none, none]`
    ///    - segment is `d`, [is_vec] is true -> `[]`
    fn value_of_plain_segment(
        &self,
        data: &Arc<ArcTopicDataValue>,
        segment: &PlainDataPath,
        full_path: &String,
    ) -> StdR<Arc<ArcTopicDataValue>> {
        let current_path = &segment.path;
        let current_is_vec = &segment.is_vec.unwrap_or(false);

        match data.deref() {
            ArcTopicDataValue::Map(map) => {
                if let Some(value) = map.get(current_path) {
                    Ok(value.clone())
                } else {
                    Ok(Arc::new(ArcTopicDataValue::None))
                }
            }
            ArcTopicDataValue::Vec(vec) => {
                let mut values = vec![];
                for value in vec.iter() {
                    match value.deref() {
                        ArcTopicDataValue::None => {
                            if !current_is_vec {
                                values.push(value.clone());
                            }
                        }
                        ArcTopicDataValue::Map(map) => {
                            if let Some(value) = map.get(current_path) {
                                match value.deref() {
                                    ArcTopicDataValue::None => {
                                        if !current_is_vec {
                                            values.push(value.clone())
                                        }
                                    }
                                    ArcTopicDataValue::Vec(vec) => {
                                        // flatten
                                        vec.iter().for_each(|value| values.push(value.clone()))
                                    }
                                    _ => values.push(value.clone()),
                                }
                            } else if !current_is_vec {
                                // when value type is not array, insert a none value
                                values.push(Arc::new(ArcTopicDataValue::None))
                            }
                        }
                        _ => {
                            return PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                                "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
                                full_path, current_path, &self
                            ));
                        }
                    }
                }
                Ok(Arc::new(ArcTopicDataValue::Vec(Arc::new(values))))
            }
            _ => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Cannot retrieve[key={}, current={}] as decimal from [{:?}].",
                full_path, current_path, &self
            )),
        }
    }

    fn value_of_complex_path(&self, parsed_path: &ParsedDataPath) -> StdR<Arc<ArcTopicDataValue>> {
        let path = &parsed_path.path;
        let segments = &parsed_path.segments;
        let first_segment = &segments[0];
        match first_segment {
            DataPathSegment::Func(_) => PipelineKernelErrorCode::IncorrectDataPath.msg(format!(
                "Data path[{}] is incorrect, first segment cannot be function.",
                path
            )),
            DataPathSegment::Plain(first_segment) => {
                let data = self.get(&first_segment.path);
                // value not exists
                if data.is_none() {
                    return self.transform_none_value_for_first_segment(parsed_path);
                }

                // loop from index 1
                let mut data = data.unwrap().clone();
                let remain_count = segments.len() - 1;
                let mut current_index = 1;
                while current_index <= remain_count {
                    let segment = &segments[current_index];
                    let current_is_vec = match segment {
                        DataPathSegment::Plain(plain_segment) => {
                            data = self.value_of_plain_segment(&data, plain_segment, path)?;
                            // to identify that the returned data needs to be transformed or not
                            plain_segment.is_vec.unwrap_or(false)
                        }
                        DataPathSegment::Func(func_segment) => {
                            data =
                                VariablePredefineFunctionCaller::prepare(&self, path, func_segment)
                                    .value_of(&data)?;
                            // never mind, just keep the value which returned
                            // no need to transform
                            false
                        }
                    };

                    // recheck the data, is there none, empty vec, then there is no need to go deeper.
                    // and when current segment says the value should be a vec, convert none to empty vec
                    // and return directly
                    match data.deref() {
                        ArcTopicDataValue::None => {
                            return if current_is_vec {
                                Ok(Arc::new(ArcTopicDataValue::Vec(vec![].into())))
                            } else {
                                Ok(Arc::new(ArcTopicDataValue::None))
                            };
                        }
                        ArcTopicDataValue::Vec(vec) => {
                            if vec.is_empty() {
                                return Ok(data.clone());
                            }
                        }
                        _ => {}
                    }

                    // next loop
                    current_index = current_index + 1
                }

                // return get value
                Ok(data.clone())
            }
        }
    }
}
