use crate::{BaseDataModel, EnumId, Storable};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum FactorType {
    Sequence,
    Number,
    /// 0 & positive
    Unsigned,
    Text,
    // address
    Address,
    Continent,
    Region,
    Country,
    Province,
    City,
    District,
    Road,
    Community,
    Floor,
    ResidenceType,
    ResidentialArea,
    // contact electronic
    Email,
    Phone,
    Mobile,
    Fax,
    // date time related
    /// YYYY-MM-DD HH:mm:ss
    Datetime,
    /// YYYY-MM-DD HH:mm:ss.SSS
    FullDatetime,
    /// YYYY-MM-DD
    Date,
    /// HH:mm:ss
    Time,
    /// 4 digits
    Year,
    /// 1: first half, 2: second half
    HalfYear,
    /// 1 - 4
    Quarter,
    /// 1 - 12
    Month,
    /// 1: first half, 2: second half
    HalfMonth,
    /// 1, 2, 3
    TenDays,
    /// 0 (the partial week that precedes the first Sunday of the year) - 53 (leap year)
    WeekOfYear,
    /// 0 (the partial week that precedes the first Sunday of the year) - 5
    WeekOfMonth,
    /// 1: first half, 2: second half
    HalfWeek,
    /// 1 - 31, according to month/year
    DayOfMonth,
    /// 1 (Sunday) - 7 (Saturday)
    DayOfWeek,
    /// 1: workday, 2: weekend, 3: holiday
    DayKind,
    /// 0 - 23
    Hour,
    /// 1: work time, 2: off hours, 3: sleeping time
    HourKind,
    /// 0 - 59
    Minute,
    /// 0 - 59
    Second,
    /// 0 - 999
    Millisecond,
    /// 1, 2
    AmPm,
    // individual
    Gender,
    Occupation,
    /// YYYY-MM-DD
    DateOfBirth,
    Age,
    IdNo,
    Religion,
    Nationality,
    // organization
    BizTrade,
    BizScale,
    Boolean,
    Enum,
    Object,
    Array,
}

#[derive(Display, Serde)]
pub enum FactorIndexGroup {
    #[display = ""]
    EMPTY,
    #[display = "i-1"]
    Index1,
    #[display = "i-2"]
    Index2,
    #[display = "i-3"]
    Index3,
    #[display = "i-4"]
    Index4,
    #[display = "i-5"]
    Index5,
    #[display = "i-6"]
    Index6,
    #[display = "i-7"]
    Index7,
    #[display = "i-8"]
    Index8,
    #[display = "i-9"]
    Index9,
    #[display = "i-10"]
    Index10,
    #[display = "u-1"]
    UniqueIndex1,
    #[display = "u-2"]
    UniqueIndex2,
    #[display = "u-3"]
    UniqueIndex3,
    #[display = "u-4"]
    UniqueIndex4,
    #[display = "u-5"]
    UniqueIndex5,
    #[display = "u-6"]
    UniqueIndex6,
    #[display = "u-7"]
    UniqueIndex7,
    #[display = "u-8"]
    UniqueIndex8,
    #[display = "u-9"]
    UniqueIndex9,
    #[display = "u-10"]
    UniqueIndex10,
}

#[derive(Display, Serde)]
#[pattern = "kebab-upper"]
pub enum FactorEncryptMethod {
    #[display = "none"]
    NONE,
    Aes256Pkcs5Padding,
    Md5,
    Sha256,
    MaskMail,
    #[display = "MASK-CENTER-3"]
    MaskCenter3,
    #[display = "MASK-CENTER-5"]
    MaskCenter5,
    #[display = "MASK-LAST-3"]
    MaskLast3,
    #[display = "MASK-LAST-6"]
    MaskLast6,
    MaskDay,
    MaskMonth,
    MaskMonthDay,
}

pub type FactorId = String;

#[adapt_model(storable)]
pub struct Factor {
    pub factor_id: Option<FactorId>,
    pub r#type: Option<FactorType>,
    pub name: Option<String>,
    pub enum_id: Option<EnumId>,
    pub label: Option<String>,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub flatten: Option<bool>,
    pub index_group: Option<FactorIndexGroup>,
    pub encrypt: Option<FactorEncryptMethod>,
    pub precision: Option<String>,
}

pub struct FactorBuilder {
    factor_id: Option<FactorId>,
    r#type: Option<FactorType>,
    name: Option<String>,
    enum_id: Option<EnumId>,
    label: Option<String>,
    description: Option<String>,
    default_value: Option<String>,
    flatten: Option<bool>,
    index_group: Option<FactorIndexGroup>,
    encrypt: Option<FactorEncryptMethod>,
    precision: Option<String>,
}

impl FactorBuilder {
    pub fn new() -> Self {
        FactorBuilder {
            factor_id: None,
            r#type: None,
            name: None,
            enum_id: None,
            label: None,
            description: None,
            default_value: None,
            flatten: None,
            index_group: None,
            encrypt: None,
            precision: None,
        }
    }

    pub fn factor_id(mut self, factor_id: FactorId) -> Self {
        self.factor_id = Some(factor_id);
        self
    }

    pub fn r#type(mut self, r#type: FactorType) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn enum_id(mut self, enum_id: EnumId) -> Self {
        self.enum_id = Some(enum_id);
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn default_value(mut self, default_value: String) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn flatten(mut self, flatten: bool) -> Self {
        self.flatten = Some(flatten);
        self
    }

    pub fn index_group(mut self, index_group: FactorIndexGroup) -> Self {
        self.index_group = Some(index_group);
        self
    }

    pub fn encrypt(mut self, encrypt: FactorEncryptMethod) -> Self {
        self.encrypt = Some(encrypt);
        self
    }

    pub fn precision(mut self, precision: String) -> Self {
        self.precision = Some(precision);
        self
    }

    pub fn build(self) -> Factor {
        Factor {
            factor_id: self.factor_id,
            r#type: self.r#type,
            name: self.name,
            enum_id: self.enum_id,
            label: self.label,
            description: self.description,
            default_value: self.default_value,
            flatten: self.flatten,
            index_group: self.index_group,
            encrypt: self.encrypt,
            precision: self.precision,
        }
    }
}
