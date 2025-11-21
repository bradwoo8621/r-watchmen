use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, EnumId, MeasureMethod, OptimisticLock, Storable, TenantBasedTuple,
    TenantId, Tuple, UserId,
};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum BucketType {
    Value,
    ValueMeasure,
    CategoryMeasure,
    EnumMeasure,
}

#[derive(Display, Serde)]
pub enum RangeBucketValueIncluding {
    IncludeMin,
    IncludeMax,
}

#[adapt_model(storable)]
pub struct NumericSegmentValue {
    pub min: Option<String>,
    pub max: Option<String>,
}

#[adapt_model(storable)]
pub struct NumericValueSegment {
    pub name: Option<String>,
    pub value: Option<NumericSegmentValue>,
}

#[adapt_model(opt_lock, tenant_based)]
pub struct NumericValueBucket {
    pub bucket_id: Option<BucketId>,
    pub name: Option<String>,
    pub r#type: Option<BucketType>,
    pub description: Option<String>,
    pub include: Option<RangeBucketValueIncluding>,
    pub segments: Option<Vec<NumericValueSegment>>,
}

impl NumericValueBucket {
    pub fn init() -> Self {
        NumericValueBucket::new().r#type(BucketType::Value)
    }

    pub fn to_bucket(self) -> Bucket {
        Bucket::NumericValue(self)
    }
}

#[adapt_model(opt_lock, tenant_based)]
pub struct NumericValueMeasureBucket {
    pub bucket_id: Option<BucketId>,
    pub name: Option<String>,
    pub r#type: Option<BucketType>,
    pub description: Option<String>,
    /// can be [MeasureMethod::Floor], [MeasureMethod::ResidentialArea], [MeasureMethod::Age] or [MeasureMethod::BizScale]
    pub measure: Option<MeasureMethod>,
    pub include: Option<RangeBucketValueIncluding>,
    pub segments: Option<Vec<NumericValueSegment>>,
}

impl NumericValueMeasureBucket {
    pub fn init() -> Self {
        NumericValueMeasureBucket::new().r#type(BucketType::ValueMeasure)
    }

    pub fn with_floor() -> Self {
        NumericValueMeasureBucket::init().measure(MeasureMethod::Floor)
    }

    pub fn with_residential_area() -> Self {
        NumericValueMeasureBucket::init().measure(MeasureMethod::ResidentialArea)
    }

    pub fn with_age() -> Self {
        NumericValueMeasureBucket::init().measure(MeasureMethod::Age)
    }

    pub fn with_biz_scale() -> Self {
        NumericValueMeasureBucket::init().measure(MeasureMethod::BizScale)
    }

    pub fn to_bucket(self) -> Bucket {
        Bucket::NumericValueMeasure(self)
    }
}

pub const OTHER_CATEGORY_SEGMENT_VALUE: &'static str = "&others";
/// [OTHER_CATEGORY_SEGMENT_VALUE]
pub type CategorySegmentValue = Vec<String>;

#[adapt_model(storable)]
pub struct CategorySegment {
    pub name: Option<String>,
    pub value: Option<CategorySegmentValue>,
}

#[adapt_model(opt_lock, tenant_based)]
pub struct CategoryMeasureBucket {
    pub bucket_id: Option<BucketId>,
    pub name: Option<String>,
    pub r#type: Option<BucketType>,
    pub description: Option<String>,
    /// can be one of following:
    /// - [MeasureMethod::Continent],
    /// - [MeasureMethod::Region],
    /// - [MeasureMethod::Country],
    /// - [MeasureMethod::Province],
    /// - [MeasureMethod::City],
    /// - [MeasureMethod::District],
    /// - [MeasureMethod::ResidenceType],
    /// - [MeasureMethod::Gender],
    /// - [MeasureMethod::Occupation],
    /// - [MeasureMethod::Religion],
    /// - [MeasureMethod::Nationality],
    /// - [MeasureMethod::BizTrade],
    /// - [MeasureMethod::Boolean],
    pub measure: Option<MeasureMethod>,
    pub segments: Option<Vec<CategorySegment>>,
}

impl CategoryMeasureBucket {
    pub fn init() -> Self {
        CategoryMeasureBucket::new().r#type(BucketType::CategoryMeasure)
    }

    pub fn with_continent() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::Continent)
    }

    pub fn with_region() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::Region)
    }

    pub fn with_country() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::Country)
    }

    pub fn with_province() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::Province)
    }

    pub fn with_city() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::City)
    }

    pub fn with_district() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::District)
    }

    pub fn with_residence_type() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::ResidenceType)
    }

    pub fn with_gender() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::Gender)
    }

    pub fn with_occupation() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::Occupation)
    }

    pub fn with_religion() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::Religion)
    }

    pub fn with_nationality() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::Nationality)
    }

    pub fn with_biz_trade() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::BizTrade)
    }

    pub fn with_boolean() -> Self {
        CategoryMeasureBucket::init().measure(MeasureMethod::Boolean)
    }

    pub fn to_bucket(self) -> Bucket {
        Bucket::CategoryMeasure(self)
    }
}

#[adapt_model(opt_lock, tenant_based)]
pub struct EnumMeasureBucket {
    pub bucket_id: Option<BucketId>,
    pub name: Option<String>,
    pub r#type: Option<BucketType>,
    pub description: Option<String>,
    pub measure: Option<MeasureMethod>,
    pub segments: Option<Vec<CategorySegment>>,
    pub enum_id: Option<EnumId>,
}

impl EnumMeasureBucket {
    pub fn init() -> Self {
        EnumMeasureBucket::new()
            .r#type(BucketType::EnumMeasure)
            .measure(MeasureMethod::Enum)
    }

    pub fn to_bucket(self) -> Bucket {
        Bucket::EnumMeasure(self)
    }
}

pub type BucketId = String;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bucket {
    NumericValue(NumericValueBucket),
    NumericValueMeasure(NumericValueMeasureBucket),
    CategoryMeasure(CategoryMeasureBucket),
    EnumMeasure(EnumMeasureBucket),
}
