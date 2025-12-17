use crate::{DataPath, DataPathSegment, PathParser, PipelineKernelErrorCode, PlainDataPath};
use std::ops::Deref;
use watchmen_model::{FactorType, StdErrorCode, StdR};
use watchmen_runtime_model_kernel::{ArcFactor, TopicSchema};

impl DataPath {
    /// factor name has no dot escape, no function
    pub fn from_factor(factor: &ArcFactor, topic_schema: &TopicSchema) -> StdR<DataPath> {
        let mut segments = vec![];
        let segment_paths: Vec<&str> = factor.name.split('.').collect();
        for (index, _) in segment_paths.iter().enumerate() {
            // each path is from start
            let path = segment_paths[0..(index + 1)].join(".");
            let factor = topic_schema.factor_by_name(&path);
            let is_vec = if let Some(factor) = factor {
                *factor.r#type.as_ref() == FactorType::Array
            } else {
                return PipelineKernelErrorCode::FactorNotFound.msg(format!(
                    "Factor[{}] not found in topic[{}].",
                    &path,
                    topic_schema.topic_id()
                ));
            };
            segments.push(DataPathSegment::Plain(PlainDataPath {
                path: segment_paths[index].to_string(),
                is_vec: Some(is_vec),
            }));
        }

        Ok(DataPath {
            path: factor.name.deref().clone(),
            segments,
        })
    }

    /// all kinds escapes, functions, variables
    /// - \. escapes dot,
    /// - \, escapes comma,
    /// - \( escapes left parenthesis,
    /// - \) escapes right parenthesis,
    /// - \{ escapes left brace,
    /// - \} escapes right brace,
    /// - \& escapes ampersand,
    /// - abc{ef} escapes path [ef],
    ///
    /// and fail fast
    pub fn from_str(path: &str) -> StdR<DataPath> {
        let all_chars = path.chars().collect();
        let mut parser = PathParser::by_path(path, &all_chars);
        parser.parse()?;

        Ok(DataPath {
            path: path.to_string(),
            segments: parser.segments,
        })
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    mod helper {
        use crate::{DataPathSegment, FuncDataPathParam, FuncParamValue};

        pub fn assert_plain(segment: &DataPathSegment, value: &str) {
            assert!(matches!(segment, DataPathSegment::Plain(_)));
            match segment {
                DataPathSegment::Plain(path) => {
                    assert_eq!(path.path, value);
                    assert_eq!(path.is_vec, None);
                }
                _ => panic!(),
            }
        }

        pub fn assert_param_plain(param: &FuncDataPathParam, value: &str) {
            assert!(matches!(param, FuncDataPathParam::Plain(_)));
            match param {
                FuncDataPathParam::Plain(plain_path) => {
                    assert_eq!(plain_path.path, value);
                    assert_eq!(plain_path.is_vec, None);
                }
                _ => panic!(),
            }
        }

        pub fn assert_param_str(param: &FuncDataPathParam, value: &str) {
            match param {
                FuncDataPathParam::Value(value_path) => {
                    assert_eq!(value_path.path, value);
                    assert!(matches!(value_path.value, FuncParamValue::Str(_)));
                    match &value_path.value {
                        FuncParamValue::Str(s) => {
                            assert_eq!(s, value);
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }
    }

    mod plain {
        use crate::data_op::data_path_parser::tests::helper::assert_plain;
        use crate::DataPath;

        #[test]
        fn test__a() {
            println!("test__a");

            let path = DataPath::from_str("a").unwrap();
            assert_eq!(path.path, "a");
            assert_eq!(path.segments.len(), 1);
            assert_plain(&path.segments[0], "a");
        }

        #[test]
        fn test__a_b() {
            println!("test__a_b");

            let path = DataPath::from_str("a.b").unwrap();
            assert_eq!(path.path, "a.b");
            assert_eq!(path.segments.len(), 2);
            assert_plain(&path.segments[0], "a");
            assert_plain(&path.segments[1], "b");
        }

        #[test]
        fn test__a_b_c() {
            println!("test__a_b_c");

            let path = DataPath::from_str("a.b.c").unwrap();
            assert_eq!(path.path, "a.b.c");
            assert_eq!(path.segments.len(), 3);
            assert_plain(&path.segments[0], "a");
            assert_plain(&path.segments[1], "b");
            assert_plain(&path.segments[2], "c");
        }
    }

    mod literal_concat {
        use crate::data_op::data_path_parser::tests::helper::{assert_param_plain, assert_param_str};
        use crate::{DataPath, DataPathSegment};
        use watchmen_model::VariablePredefineFunctions;

        #[test]
        fn test__LBaRB() {
            println!("test__LBaRB");

            let path = DataPath::from_str("{a}").unwrap();
            assert_eq!(path.path, "{a}");
            assert_eq!(path.segments.len(), 1);
            assert!(matches!(&path.segments[0], DataPathSegment::Func(_)));
            match &path.segments[0] {
                DataPathSegment::Func(func_path) => {
                    assert_eq!(func_path.path, "{a}");
                    assert!(matches!(
                        &func_path.func,
                        VariablePredefineFunctions::Concat
                    ));
                    assert!(func_path.params.is_some());
                    if let Some(params) = &func_path.params {
                        assert_eq!(params.len(), 1);
                        assert_param_plain(&params[0], "a");
                    }
                }
                _ => {}
            }
        }

        #[test]
        fn test__LBaRBb() {
            println!("test__LBaRBb");

            let path = DataPath::from_str("{a}b").unwrap();
            assert_eq!(path.path, "{a}b");
            assert_eq!(path.segments.len(), 1);
            assert!(matches!(&path.segments[0], DataPathSegment::Func(_)));
            match &path.segments[0] {
                DataPathSegment::Func(func_path) => {
                    assert_eq!(func_path.path, "{a}b");
                    assert!(matches!(
                        &func_path.func,
                        VariablePredefineFunctions::Concat
                    ));
                    assert!(func_path.params.is_some());
                    if let Some(params) = &func_path.params {
                        assert_eq!(params.len(), 2);
                        assert_param_plain(&params[0], "a");
                        assert_param_str(&params[1], "b");
                    }
                }
                _ => {}
            }
        }
    }
}
