# ! [ doc = "Generated for FIT SDK profile version: " ] # ! [ doc = "20.66.00" ]use byteorder::ByteOrder;
use error;
use profile;
#[derive(Debug)]
pub enum File {
    #[doc = "Read only, single file. Must be in root directory."]
    Device = 1,
    #[doc = "Read/write, single file. Directory=Settings"]
    Settings = 2,
    #[doc = "Read/write, multiple files, file number = sport type. \
             Directory=Sports"]
    Sport = 3,
    #[doc = "Read/erase, multiple files. Directory=Activities"]
    Activity = 4,
    #[doc = "Read/write/erase, multiple files. Directory=Workouts"]
    Workout = 5,
    #[doc = "Read/write/erase, multiple files. Directory=Courses"]
    Course = 6,
    #[doc = "Read/write, single file. Directory=Schedules"]
    Schedules = 7,
    #[doc = "Read only, single file. Circular buffer. All message definitions \
             at start of file. Directory=Weight"]
    Weight = 9,
    #[doc = "Read only, single file. Directory=Totals"]
    Totals = 10,
    #[doc = "Read/write, single file. Directory=Goals"]
    Goals = 11,
    #[doc = "Read only. Directory=Blood Pressure"]
    BloodPressure = 14,
    #[doc = "Read only. Directory=Monitoring. File number=sub type."]
    MonitoringA = 15,
    #[doc = "Read/erase, multiple files. Directory=Activities"]
    ActivitySummary = 20,
    MonitoringDaily = 28,
    #[doc = "Read only. Directory=Monitoring. File number=identifier"]
    MonitoringB = 32,
    #[doc = "Read/write/erase. Multiple Files.  Directory=Segments"]
    Segment = 34,
    #[doc = "Read/write/erase. Single File.  Directory=Segments"]
    SegmentList = 35,
    #[doc = "Read/write/erase. Single File. Directory=Settings"]
    ExdConfiguration = 40,
    #[doc = "0xF7 - 0xFE reserved for manufacturer specific file types"]
    MfgRangeMin = 247,
    #[doc = "0xF7 - 0xFE reserved for manufacturer specific file types"]
    MfgRangeMax = 254,
    Unknown,
}
impl File {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(File::Device),
            2 => Ok(File::Settings),
            3 => Ok(File::Sport),
            4 => Ok(File::Activity),
            5 => Ok(File::Workout),
            6 => Ok(File::Course),
            7 => Ok(File::Schedules),
            9 => Ok(File::Weight),
            10 => Ok(File::Totals),
            11 => Ok(File::Goals),
            14 => Ok(File::BloodPressure),
            15 => Ok(File::MonitoringA),
            20 => Ok(File::ActivitySummary),
            28 => Ok(File::MonitoringDaily),
            32 => Ok(File::MonitoringB),
            34 => Ok(File::Segment),
            35 => Ok(File::SegmentList),
            40 => Ok(File::ExdConfiguration),
            247 => Ok(File::MfgRangeMin),
            254 => Ok(File::MfgRangeMax),
            _ => Ok(File::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum MesgNum {
    FileId = 0,
    Capabilities = 1,
    DeviceSettings = 2,
    UserProfile = 3,
    HrmProfile = 4,
    SdmProfile = 5,
    BikeProfile = 6,
    ZonesTarget = 7,
    HrZone = 8,
    PowerZone = 9,
    MetZone = 10,
    Sport = 12,
    Goal = 15,
    Session = 18,
    Lap = 19,
    Record = 20,
    Event = 21,
    DeviceInfo = 23,
    Workout = 26,
    WorkoutStep = 27,
    Schedule = 28,
    WeightScale = 30,
    Course = 31,
    CoursePoint = 32,
    Totals = 33,
    Activity = 34,
    Software = 35,
    FileCapabilities = 37,
    MesgCapabilities = 38,
    FieldCapabilities = 39,
    FileCreator = 49,
    BloodPressure = 51,
    SpeedZone = 53,
    Monitoring = 55,
    TrainingFile = 72,
    Hrv = 78,
    AntRx = 80,
    AntTx = 81,
    AntChannelId = 82,
    Length = 101,
    MonitoringInfo = 103,
    Pad = 105,
    SlaveDevice = 106,
    Connectivity = 127,
    WeatherConditions = 128,
    WeatherAlert = 129,
    CadenceZone = 131,
    Hr = 132,
    SegmentLap = 142,
    MemoGlob = 145,
    SegmentId = 148,
    SegmentLeaderboardEntry = 149,
    SegmentPoint = 150,
    SegmentFile = 151,
    WorkoutSession = 158,
    WatchfaceSettings = 159,
    GpsMetadata = 160,
    CameraEvent = 161,
    TimestampCorrelation = 162,
    GyroscopeData = 164,
    AccelerometerData = 165,
    ThreeDSensorCalibration = 167,
    VideoFrame = 169,
    ObdiiData = 174,
    NmeaSentence = 177,
    AviationAttitude = 178,
    Video = 184,
    VideoTitle = 185,
    VideoDescription = 186,
    VideoClip = 187,
    OhrSettings = 188,
    ExdScreenConfiguration = 200,
    ExdDataFieldConfiguration = 201,
    ExdDataConceptConfiguration = 202,
    FieldDescription = 206,
    DeveloperDataId = 207,
    MagnetometerData = 208,
    BarometerData = 209,
    OneDSensorCalibration = 210,
    Set = 225,
    StressLevel = 227,
    DiveSettings = 258,
    DiveGas = 259,
    DiveAlarm = 262,
    ExerciseTitle = 264,
    DiveSummary = 268,
    #[doc = "0xFF00 - 0xFFFE reserved for manufacturer specific messages"]
    MfgRangeMin = 65280,
    #[doc = "0xFF00 - 0xFFFE reserved for manufacturer specific messages"]
    MfgRangeMax = 65534,
    Unknown,
}
impl MesgNum {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(MesgNum::FileId),
            1 => Ok(MesgNum::Capabilities),
            2 => Ok(MesgNum::DeviceSettings),
            3 => Ok(MesgNum::UserProfile),
            4 => Ok(MesgNum::HrmProfile),
            5 => Ok(MesgNum::SdmProfile),
            6 => Ok(MesgNum::BikeProfile),
            7 => Ok(MesgNum::ZonesTarget),
            8 => Ok(MesgNum::HrZone),
            9 => Ok(MesgNum::PowerZone),
            10 => Ok(MesgNum::MetZone),
            12 => Ok(MesgNum::Sport),
            15 => Ok(MesgNum::Goal),
            18 => Ok(MesgNum::Session),
            19 => Ok(MesgNum::Lap),
            20 => Ok(MesgNum::Record),
            21 => Ok(MesgNum::Event),
            23 => Ok(MesgNum::DeviceInfo),
            26 => Ok(MesgNum::Workout),
            27 => Ok(MesgNum::WorkoutStep),
            28 => Ok(MesgNum::Schedule),
            30 => Ok(MesgNum::WeightScale),
            31 => Ok(MesgNum::Course),
            32 => Ok(MesgNum::CoursePoint),
            33 => Ok(MesgNum::Totals),
            34 => Ok(MesgNum::Activity),
            35 => Ok(MesgNum::Software),
            37 => Ok(MesgNum::FileCapabilities),
            38 => Ok(MesgNum::MesgCapabilities),
            39 => Ok(MesgNum::FieldCapabilities),
            49 => Ok(MesgNum::FileCreator),
            51 => Ok(MesgNum::BloodPressure),
            53 => Ok(MesgNum::SpeedZone),
            55 => Ok(MesgNum::Monitoring),
            72 => Ok(MesgNum::TrainingFile),
            78 => Ok(MesgNum::Hrv),
            80 => Ok(MesgNum::AntRx),
            81 => Ok(MesgNum::AntTx),
            82 => Ok(MesgNum::AntChannelId),
            101 => Ok(MesgNum::Length),
            103 => Ok(MesgNum::MonitoringInfo),
            105 => Ok(MesgNum::Pad),
            106 => Ok(MesgNum::SlaveDevice),
            127 => Ok(MesgNum::Connectivity),
            128 => Ok(MesgNum::WeatherConditions),
            129 => Ok(MesgNum::WeatherAlert),
            131 => Ok(MesgNum::CadenceZone),
            132 => Ok(MesgNum::Hr),
            142 => Ok(MesgNum::SegmentLap),
            145 => Ok(MesgNum::MemoGlob),
            148 => Ok(MesgNum::SegmentId),
            149 => Ok(MesgNum::SegmentLeaderboardEntry),
            150 => Ok(MesgNum::SegmentPoint),
            151 => Ok(MesgNum::SegmentFile),
            158 => Ok(MesgNum::WorkoutSession),
            159 => Ok(MesgNum::WatchfaceSettings),
            160 => Ok(MesgNum::GpsMetadata),
            161 => Ok(MesgNum::CameraEvent),
            162 => Ok(MesgNum::TimestampCorrelation),
            164 => Ok(MesgNum::GyroscopeData),
            165 => Ok(MesgNum::AccelerometerData),
            167 => Ok(MesgNum::ThreeDSensorCalibration),
            169 => Ok(MesgNum::VideoFrame),
            174 => Ok(MesgNum::ObdiiData),
            177 => Ok(MesgNum::NmeaSentence),
            178 => Ok(MesgNum::AviationAttitude),
            184 => Ok(MesgNum::Video),
            185 => Ok(MesgNum::VideoTitle),
            186 => Ok(MesgNum::VideoDescription),
            187 => Ok(MesgNum::VideoClip),
            188 => Ok(MesgNum::OhrSettings),
            200 => Ok(MesgNum::ExdScreenConfiguration),
            201 => Ok(MesgNum::ExdDataFieldConfiguration),
            202 => Ok(MesgNum::ExdDataConceptConfiguration),
            206 => Ok(MesgNum::FieldDescription),
            207 => Ok(MesgNum::DeveloperDataId),
            208 => Ok(MesgNum::MagnetometerData),
            209 => Ok(MesgNum::BarometerData),
            210 => Ok(MesgNum::OneDSensorCalibration),
            225 => Ok(MesgNum::Set),
            227 => Ok(MesgNum::StressLevel),
            258 => Ok(MesgNum::DiveSettings),
            259 => Ok(MesgNum::DiveGas),
            262 => Ok(MesgNum::DiveAlarm),
            264 => Ok(MesgNum::ExerciseTitle),
            268 => Ok(MesgNum::DiveSummary),
            65280 => Ok(MesgNum::MfgRangeMin),
            65534 => Ok(MesgNum::MfgRangeMax),
            _ => Ok(MesgNum::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Checksum {
    #[doc = "Allows clear of checksum for flash memory where can only write 1 \
             to 0 without erasing sector."]
    Clear = 0,
    #[doc = "Set to mark checksum as valid if computes to invalid values 0 or \
             0xFF.  Checksum can also be set to ok to save encoding \
             computation time."]
    Ok = 1,
    Unknown,
}
impl Checksum {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Checksum::Clear),
            1 => Ok(Checksum::Ok),
            _ => Ok(Checksum::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum FileFlags {
    Read = 2,
    Write = 4,
    Erase = 8,
    Unknown,
}
impl FileFlags {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            2 => Ok(FileFlags::Read),
            4 => Ok(FileFlags::Write),
            8 => Ok(FileFlags::Erase),
            _ => Ok(FileFlags::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum MesgCount {
    NumPerFile = 0,
    MaxPerFile = 1,
    MaxPerFileType = 2,
    Unknown,
}
impl MesgCount {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(MesgCount::NumPerFile),
            1 => Ok(MesgCount::MaxPerFile),
            2 => Ok(MesgCount::MaxPerFileType),
            _ => Ok(MesgCount::Unknown),
        }
    }
}
#[doc = "seconds since UTC 00:00 Dec 31 1989; if date_time is < 0x10000000 \
         then it is system time (seconds from device power on)"]
#[derive(Debug)]
pub struct DateTime(pub u32);
impl DateTime {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        Ok(DateTime(T::read_u32(buffer)))
    }
}
#[doc = "seconds since 00:00 Dec 31 1989 in local time zone; if date_time is < \
         0x10000000 then it is system time (seconds from device power on)"]
#[derive(Debug)]
pub struct LocalDateTime(pub u32);
impl LocalDateTime {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        Ok(LocalDateTime(T::read_u32(buffer)))
    }
}
#[derive(Debug)]
pub enum MessageIndex {
    #[doc = "message is selected if set"]
    Selected = 32768,
    #[doc = "reserved (default 0)"]
    Reserved = 28672,
    #[doc = "index"]
    Mask = 4095,
    Unknown,
}
impl MessageIndex {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            32768 => Ok(MessageIndex::Selected),
            28672 => Ok(MessageIndex::Reserved),
            4095 => Ok(MessageIndex::Mask),
            _ => Ok(MessageIndex::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DeviceIndex {
    #[doc = "Creator of the file is always device index 0."]
    Creator = 0,
    Unknown,
}
impl DeviceIndex {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DeviceIndex::Creator),
            _ => Ok(DeviceIndex::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Gender {
    Female = 0,
    Male = 1,
    Unknown,
}
impl Gender {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Gender::Female),
            1 => Ok(Gender::Male),
            _ => Ok(Gender::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Language {
    English = 0,
    French = 1,
    Italian = 2,
    German = 3,
    Spanish = 4,
    Croatian = 5,
    Czech = 6,
    Danish = 7,
    Dutch = 8,
    Finnish = 9,
    Greek = 10,
    Hungarian = 11,
    Norwegian = 12,
    Polish = 13,
    Portuguese = 14,
    Slovakian = 15,
    Slovenian = 16,
    Swedish = 17,
    Russian = 18,
    Turkish = 19,
    Latvian = 20,
    Ukrainian = 21,
    Arabic = 22,
    Farsi = 23,
    Bulgarian = 24,
    Romanian = 25,
    Chinese = 26,
    Japanese = 27,
    Korean = 28,
    Taiwanese = 29,
    Thai = 30,
    Hebrew = 31,
    BrazilianPortuguese = 32,
    Indonesian = 33,
    Malaysian = 34,
    Vietnamese = 35,
    Burmese = 36,
    Mongolian = 37,
    Custom = 254,
    Unknown,
}
impl Language {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Language::English),
            1 => Ok(Language::French),
            2 => Ok(Language::Italian),
            3 => Ok(Language::German),
            4 => Ok(Language::Spanish),
            5 => Ok(Language::Croatian),
            6 => Ok(Language::Czech),
            7 => Ok(Language::Danish),
            8 => Ok(Language::Dutch),
            9 => Ok(Language::Finnish),
            10 => Ok(Language::Greek),
            11 => Ok(Language::Hungarian),
            12 => Ok(Language::Norwegian),
            13 => Ok(Language::Polish),
            14 => Ok(Language::Portuguese),
            15 => Ok(Language::Slovakian),
            16 => Ok(Language::Slovenian),
            17 => Ok(Language::Swedish),
            18 => Ok(Language::Russian),
            19 => Ok(Language::Turkish),
            20 => Ok(Language::Latvian),
            21 => Ok(Language::Ukrainian),
            22 => Ok(Language::Arabic),
            23 => Ok(Language::Farsi),
            24 => Ok(Language::Bulgarian),
            25 => Ok(Language::Romanian),
            26 => Ok(Language::Chinese),
            27 => Ok(Language::Japanese),
            28 => Ok(Language::Korean),
            29 => Ok(Language::Taiwanese),
            30 => Ok(Language::Thai),
            31 => Ok(Language::Hebrew),
            32 => Ok(Language::BrazilianPortuguese),
            33 => Ok(Language::Indonesian),
            34 => Ok(Language::Malaysian),
            35 => Ok(Language::Vietnamese),
            36 => Ok(Language::Burmese),
            37 => Ok(Language::Mongolian),
            254 => Ok(Language::Custom),
            _ => Ok(Language::Unknown),
        }
    }
}
#[doc = "Bit field corresponding to language enum type (1 << language)."]
#[derive(Debug)]
pub enum LanguageBits0 {
    English = 1,
    French = 2,
    Italian = 4,
    German = 8,
    Spanish = 16,
    Croatian = 32,
    Czech = 64,
    Danish = 128,
    Unknown,
}
impl LanguageBits0 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(LanguageBits0::English),
            2 => Ok(LanguageBits0::French),
            4 => Ok(LanguageBits0::Italian),
            8 => Ok(LanguageBits0::German),
            16 => Ok(LanguageBits0::Spanish),
            32 => Ok(LanguageBits0::Croatian),
            64 => Ok(LanguageBits0::Czech),
            128 => Ok(LanguageBits0::Danish),
            _ => Ok(LanguageBits0::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LanguageBits1 {
    Dutch = 1,
    Finnish = 2,
    Greek = 4,
    Hungarian = 8,
    Norwegian = 16,
    Polish = 32,
    Portuguese = 64,
    Slovakian = 128,
    Unknown,
}
impl LanguageBits1 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(LanguageBits1::Dutch),
            2 => Ok(LanguageBits1::Finnish),
            4 => Ok(LanguageBits1::Greek),
            8 => Ok(LanguageBits1::Hungarian),
            16 => Ok(LanguageBits1::Norwegian),
            32 => Ok(LanguageBits1::Polish),
            64 => Ok(LanguageBits1::Portuguese),
            128 => Ok(LanguageBits1::Slovakian),
            _ => Ok(LanguageBits1::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LanguageBits2 {
    Slovenian = 1,
    Swedish = 2,
    Russian = 4,
    Turkish = 8,
    Latvian = 16,
    Ukrainian = 32,
    Arabic = 64,
    Farsi = 128,
    Unknown,
}
impl LanguageBits2 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(LanguageBits2::Slovenian),
            2 => Ok(LanguageBits2::Swedish),
            4 => Ok(LanguageBits2::Russian),
            8 => Ok(LanguageBits2::Turkish),
            16 => Ok(LanguageBits2::Latvian),
            32 => Ok(LanguageBits2::Ukrainian),
            64 => Ok(LanguageBits2::Arabic),
            128 => Ok(LanguageBits2::Farsi),
            _ => Ok(LanguageBits2::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LanguageBits3 {
    Bulgarian = 1,
    Romanian = 2,
    Chinese = 4,
    Japanese = 8,
    Korean = 16,
    Taiwanese = 32,
    Thai = 64,
    Hebrew = 128,
    Unknown,
}
impl LanguageBits3 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(LanguageBits3::Bulgarian),
            2 => Ok(LanguageBits3::Romanian),
            4 => Ok(LanguageBits3::Chinese),
            8 => Ok(LanguageBits3::Japanese),
            16 => Ok(LanguageBits3::Korean),
            32 => Ok(LanguageBits3::Taiwanese),
            64 => Ok(LanguageBits3::Thai),
            128 => Ok(LanguageBits3::Hebrew),
            _ => Ok(LanguageBits3::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LanguageBits4 {
    BrazilianPortuguese = 1,
    Indonesian = 2,
    Malaysian = 4,
    Vietnamese = 8,
    Burmese = 16,
    Mongolian = 32,
    Unknown,
}
impl LanguageBits4 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(LanguageBits4::BrazilianPortuguese),
            2 => Ok(LanguageBits4::Indonesian),
            4 => Ok(LanguageBits4::Malaysian),
            8 => Ok(LanguageBits4::Vietnamese),
            16 => Ok(LanguageBits4::Burmese),
            32 => Ok(LanguageBits4::Mongolian),
            _ => Ok(LanguageBits4::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum TimeZone {
    Almaty = 0,
    Bangkok = 1,
    Bombay = 2,
    Brasilia = 3,
    Cairo = 4,
    CapeVerdeIs = 5,
    Darwin = 6,
    Eniwetok = 7,
    Fiji = 8,
    HongKong = 9,
    Islamabad = 10,
    Kabul = 11,
    Magadan = 12,
    MidAtlantic = 13,
    Moscow = 14,
    Muscat = 15,
    Newfoundland = 16,
    Samoa = 17,
    Sydney = 18,
    Tehran = 19,
    Tokyo = 20,
    UsAlaska = 21,
    UsAtlantic = 22,
    UsCentral = 23,
    UsEastern = 24,
    UsHawaii = 25,
    UsMountain = 26,
    UsPacific = 27,
    Other = 28,
    Auckland = 29,
    Kathmandu = 30,
    EuropeWesternWet = 31,
    EuropeCentralCet = 32,
    EuropeEasternEet = 33,
    Jakarta = 34,
    Perth = 35,
    Adelaide = 36,
    Brisbane = 37,
    Tasmania = 38,
    Iceland = 39,
    Amsterdam = 40,
    Athens = 41,
    Barcelona = 42,
    Berlin = 43,
    Brussels = 44,
    Budapest = 45,
    Copenhagen = 46,
    Dublin = 47,
    Helsinki = 48,
    Lisbon = 49,
    London = 50,
    Madrid = 51,
    Munich = 52,
    Oslo = 53,
    Paris = 54,
    Prague = 55,
    Reykjavik = 56,
    Rome = 57,
    Stockholm = 58,
    Vienna = 59,
    Warsaw = 60,
    Zurich = 61,
    Quebec = 62,
    Ontario = 63,
    Manitoba = 64,
    Saskatchewan = 65,
    Alberta = 66,
    BritishColumbia = 67,
    Boise = 68,
    Boston = 69,
    Chicago = 70,
    Dallas = 71,
    Denver = 72,
    KansasCity = 73,
    LasVegas = 74,
    LosAngeles = 75,
    Miami = 76,
    Minneapolis = 77,
    NewYork = 78,
    NewOrleans = 79,
    Phoenix = 80,
    SantaFe = 81,
    Seattle = 82,
    WashingtonDc = 83,
    UsArizona = 84,
    Chita = 85,
    Ekaterinburg = 86,
    Irkutsk = 87,
    Kaliningrad = 88,
    Krasnoyarsk = 89,
    Novosibirsk = 90,
    PetropavlovskKamchatskiy = 91,
    Samara = 92,
    Vladivostok = 93,
    MexicoCentral = 94,
    MexicoMountain = 95,
    MexicoPacific = 96,
    CapeTown = 97,
    Winkhoek = 98,
    Lagos = 99,
    Riyahd = 100,
    Venezuela = 101,
    AustraliaLh = 102,
    Santiago = 103,
    Manual = 253,
    Automatic = 254,
    Unknown,
}
impl TimeZone {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(TimeZone::Almaty),
            1 => Ok(TimeZone::Bangkok),
            2 => Ok(TimeZone::Bombay),
            3 => Ok(TimeZone::Brasilia),
            4 => Ok(TimeZone::Cairo),
            5 => Ok(TimeZone::CapeVerdeIs),
            6 => Ok(TimeZone::Darwin),
            7 => Ok(TimeZone::Eniwetok),
            8 => Ok(TimeZone::Fiji),
            9 => Ok(TimeZone::HongKong),
            10 => Ok(TimeZone::Islamabad),
            11 => Ok(TimeZone::Kabul),
            12 => Ok(TimeZone::Magadan),
            13 => Ok(TimeZone::MidAtlantic),
            14 => Ok(TimeZone::Moscow),
            15 => Ok(TimeZone::Muscat),
            16 => Ok(TimeZone::Newfoundland),
            17 => Ok(TimeZone::Samoa),
            18 => Ok(TimeZone::Sydney),
            19 => Ok(TimeZone::Tehran),
            20 => Ok(TimeZone::Tokyo),
            21 => Ok(TimeZone::UsAlaska),
            22 => Ok(TimeZone::UsAtlantic),
            23 => Ok(TimeZone::UsCentral),
            24 => Ok(TimeZone::UsEastern),
            25 => Ok(TimeZone::UsHawaii),
            26 => Ok(TimeZone::UsMountain),
            27 => Ok(TimeZone::UsPacific),
            28 => Ok(TimeZone::Other),
            29 => Ok(TimeZone::Auckland),
            30 => Ok(TimeZone::Kathmandu),
            31 => Ok(TimeZone::EuropeWesternWet),
            32 => Ok(TimeZone::EuropeCentralCet),
            33 => Ok(TimeZone::EuropeEasternEet),
            34 => Ok(TimeZone::Jakarta),
            35 => Ok(TimeZone::Perth),
            36 => Ok(TimeZone::Adelaide),
            37 => Ok(TimeZone::Brisbane),
            38 => Ok(TimeZone::Tasmania),
            39 => Ok(TimeZone::Iceland),
            40 => Ok(TimeZone::Amsterdam),
            41 => Ok(TimeZone::Athens),
            42 => Ok(TimeZone::Barcelona),
            43 => Ok(TimeZone::Berlin),
            44 => Ok(TimeZone::Brussels),
            45 => Ok(TimeZone::Budapest),
            46 => Ok(TimeZone::Copenhagen),
            47 => Ok(TimeZone::Dublin),
            48 => Ok(TimeZone::Helsinki),
            49 => Ok(TimeZone::Lisbon),
            50 => Ok(TimeZone::London),
            51 => Ok(TimeZone::Madrid),
            52 => Ok(TimeZone::Munich),
            53 => Ok(TimeZone::Oslo),
            54 => Ok(TimeZone::Paris),
            55 => Ok(TimeZone::Prague),
            56 => Ok(TimeZone::Reykjavik),
            57 => Ok(TimeZone::Rome),
            58 => Ok(TimeZone::Stockholm),
            59 => Ok(TimeZone::Vienna),
            60 => Ok(TimeZone::Warsaw),
            61 => Ok(TimeZone::Zurich),
            62 => Ok(TimeZone::Quebec),
            63 => Ok(TimeZone::Ontario),
            64 => Ok(TimeZone::Manitoba),
            65 => Ok(TimeZone::Saskatchewan),
            66 => Ok(TimeZone::Alberta),
            67 => Ok(TimeZone::BritishColumbia),
            68 => Ok(TimeZone::Boise),
            69 => Ok(TimeZone::Boston),
            70 => Ok(TimeZone::Chicago),
            71 => Ok(TimeZone::Dallas),
            72 => Ok(TimeZone::Denver),
            73 => Ok(TimeZone::KansasCity),
            74 => Ok(TimeZone::LasVegas),
            75 => Ok(TimeZone::LosAngeles),
            76 => Ok(TimeZone::Miami),
            77 => Ok(TimeZone::Minneapolis),
            78 => Ok(TimeZone::NewYork),
            79 => Ok(TimeZone::NewOrleans),
            80 => Ok(TimeZone::Phoenix),
            81 => Ok(TimeZone::SantaFe),
            82 => Ok(TimeZone::Seattle),
            83 => Ok(TimeZone::WashingtonDc),
            84 => Ok(TimeZone::UsArizona),
            85 => Ok(TimeZone::Chita),
            86 => Ok(TimeZone::Ekaterinburg),
            87 => Ok(TimeZone::Irkutsk),
            88 => Ok(TimeZone::Kaliningrad),
            89 => Ok(TimeZone::Krasnoyarsk),
            90 => Ok(TimeZone::Novosibirsk),
            91 => Ok(TimeZone::PetropavlovskKamchatskiy),
            92 => Ok(TimeZone::Samara),
            93 => Ok(TimeZone::Vladivostok),
            94 => Ok(TimeZone::MexicoCentral),
            95 => Ok(TimeZone::MexicoMountain),
            96 => Ok(TimeZone::MexicoPacific),
            97 => Ok(TimeZone::CapeTown),
            98 => Ok(TimeZone::Winkhoek),
            99 => Ok(TimeZone::Lagos),
            100 => Ok(TimeZone::Riyahd),
            101 => Ok(TimeZone::Venezuela),
            102 => Ok(TimeZone::AustraliaLh),
            103 => Ok(TimeZone::Santiago),
            253 => Ok(TimeZone::Manual),
            254 => Ok(TimeZone::Automatic),
            _ => Ok(TimeZone::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DisplayMeasure {
    Metric = 0,
    Statute = 1,
    Nautical = 2,
    Unknown,
}
impl DisplayMeasure {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DisplayMeasure::Metric),
            1 => Ok(DisplayMeasure::Statute),
            2 => Ok(DisplayMeasure::Nautical),
            _ => Ok(DisplayMeasure::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DisplayHeart {
    Bpm = 0,
    Max = 1,
    Reserve = 2,
    Unknown,
}
impl DisplayHeart {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DisplayHeart::Bpm),
            1 => Ok(DisplayHeart::Max),
            2 => Ok(DisplayHeart::Reserve),
            _ => Ok(DisplayHeart::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DisplayPower {
    Watts = 0,
    PercentFtp = 1,
    Unknown,
}
impl DisplayPower {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DisplayPower::Watts),
            1 => Ok(DisplayPower::PercentFtp),
            _ => Ok(DisplayPower::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DisplayPosition {
    #[doc = "dd.dddddd"]
    Degree = 0,
    #[doc = "dddmm.mmm"]
    DegreeMinute = 1,
    #[doc = "dddmmss"]
    DegreeMinuteSecond = 2,
    #[doc = "Austrian Grid (BMN)"]
    AustrianGrid = 3,
    #[doc = "British National Grid"]
    BritishGrid = 4,
    #[doc = "Dutch grid system"]
    DutchGrid = 5,
    #[doc = "Hungarian grid system"]
    HungarianGrid = 6,
    #[doc = "Finnish grid system Zone3 KKJ27"]
    FinnishGrid = 7,
    #[doc = "Gausss Krueger (German)"]
    GermanGrid = 8,
    #[doc = "Icelandic Grid"]
    IcelandicGrid = 9,
    #[doc = "Indonesian Equatorial LCO"]
    IndonesianEquatorial = 10,
    #[doc = "Indonesian Irian LCO"]
    IndonesianIrian = 11,
    #[doc = "Indonesian Southern LCO"]
    IndonesianSouthern = 12,
    #[doc = "India zone 0"]
    IndiaZone0 = 13,
    #[doc = "India zone IA"]
    IndiaZoneIA = 14,
    #[doc = "India zone IB"]
    IndiaZoneIB = 15,
    #[doc = "India zone IIA"]
    IndiaZoneIIA = 16,
    #[doc = "India zone IIB"]
    IndiaZoneIIB = 17,
    #[doc = "India zone IIIA"]
    IndiaZoneIIIA = 18,
    #[doc = "India zone IIIB"]
    IndiaZoneIIIB = 19,
    #[doc = "India zone IVA"]
    IndiaZoneIVA = 20,
    #[doc = "India zone IVB"]
    IndiaZoneIVB = 21,
    #[doc = "Irish Transverse Mercator"]
    IrishTransverse = 22,
    #[doc = "Irish Grid"]
    IrishGrid = 23,
    #[doc = "Loran TD"]
    Loran = 24,
    #[doc = "Maidenhead grid system"]
    MaidenheadGrid = 25,
    #[doc = "MGRS grid system"]
    MgrsGrid = 26,
    #[doc = "New Zealand grid system"]
    NewZealandGrid = 27,
    #[doc = "New Zealand Transverse Mercator"]
    NewZealandTransverse = 28,
    #[doc = "Qatar National Grid"]
    QatarGrid = 29,
    #[doc = "Modified RT-90 (Sweden)"]
    ModifiedSwedishGrid = 30,
    #[doc = "RT-90 (Sweden)"]
    SwedishGrid = 31,
    #[doc = "South African Grid"]
    SouthAfricanGrid = 32,
    #[doc = "Swiss CH-1903 grid"]
    SwissGrid = 33,
    #[doc = "Taiwan Grid"]
    TaiwanGrid = 34,
    #[doc = "United States National Grid"]
    UnitedStatesGrid = 35,
    #[doc = "UTM/UPS grid system"]
    UtmUpsGrid = 36,
    #[doc = "West Malayan RSO"]
    WestMalayan = 37,
    #[doc = "Borneo RSO"]
    BorneoRso = 38,
    #[doc = "Estonian grid system"]
    EstonianGrid = 39,
    #[doc = "Latvian Transverse Mercator"]
    LatvianGrid = 40,
    #[doc = "Reference Grid 99 TM (Swedish)"]
    SwedishRef99Grid = 41,
    Unknown,
}
impl DisplayPosition {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DisplayPosition::Degree),
            1 => Ok(DisplayPosition::DegreeMinute),
            2 => Ok(DisplayPosition::DegreeMinuteSecond),
            3 => Ok(DisplayPosition::AustrianGrid),
            4 => Ok(DisplayPosition::BritishGrid),
            5 => Ok(DisplayPosition::DutchGrid),
            6 => Ok(DisplayPosition::HungarianGrid),
            7 => Ok(DisplayPosition::FinnishGrid),
            8 => Ok(DisplayPosition::GermanGrid),
            9 => Ok(DisplayPosition::IcelandicGrid),
            10 => Ok(DisplayPosition::IndonesianEquatorial),
            11 => Ok(DisplayPosition::IndonesianIrian),
            12 => Ok(DisplayPosition::IndonesianSouthern),
            13 => Ok(DisplayPosition::IndiaZone0),
            14 => Ok(DisplayPosition::IndiaZoneIA),
            15 => Ok(DisplayPosition::IndiaZoneIB),
            16 => Ok(DisplayPosition::IndiaZoneIIA),
            17 => Ok(DisplayPosition::IndiaZoneIIB),
            18 => Ok(DisplayPosition::IndiaZoneIIIA),
            19 => Ok(DisplayPosition::IndiaZoneIIIB),
            20 => Ok(DisplayPosition::IndiaZoneIVA),
            21 => Ok(DisplayPosition::IndiaZoneIVB),
            22 => Ok(DisplayPosition::IrishTransverse),
            23 => Ok(DisplayPosition::IrishGrid),
            24 => Ok(DisplayPosition::Loran),
            25 => Ok(DisplayPosition::MaidenheadGrid),
            26 => Ok(DisplayPosition::MgrsGrid),
            27 => Ok(DisplayPosition::NewZealandGrid),
            28 => Ok(DisplayPosition::NewZealandTransverse),
            29 => Ok(DisplayPosition::QatarGrid),
            30 => Ok(DisplayPosition::ModifiedSwedishGrid),
            31 => Ok(DisplayPosition::SwedishGrid),
            32 => Ok(DisplayPosition::SouthAfricanGrid),
            33 => Ok(DisplayPosition::SwissGrid),
            34 => Ok(DisplayPosition::TaiwanGrid),
            35 => Ok(DisplayPosition::UnitedStatesGrid),
            36 => Ok(DisplayPosition::UtmUpsGrid),
            37 => Ok(DisplayPosition::WestMalayan),
            38 => Ok(DisplayPosition::BorneoRso),
            39 => Ok(DisplayPosition::EstonianGrid),
            40 => Ok(DisplayPosition::LatvianGrid),
            41 => Ok(DisplayPosition::SwedishRef99Grid),
            _ => Ok(DisplayPosition::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Switch {
    Off = 0,
    On = 1,
    Auto = 2,
    Unknown,
}
impl Switch {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Switch::Off),
            1 => Ok(Switch::On),
            2 => Ok(Switch::Auto),
            _ => Ok(Switch::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Sport {
    Generic = 0,
    Running = 1,
    Cycling = 2,
    #[doc = "Mulitsport transition"]
    Transition = 3,
    FitnessEquipment = 4,
    Swimming = 5,
    Basketball = 6,
    Soccer = 7,
    Tennis = 8,
    AmericanFootball = 9,
    Training = 10,
    Walking = 11,
    CrossCountrySkiing = 12,
    AlpineSkiing = 13,
    Snowboarding = 14,
    Rowing = 15,
    Mountaineering = 16,
    Hiking = 17,
    Multisport = 18,
    Paddling = 19,
    Flying = 20,
    EBiking = 21,
    Motorcycling = 22,
    Boating = 23,
    Driving = 24,
    Golf = 25,
    HangGliding = 26,
    HorsebackRiding = 27,
    Hunting = 28,
    Fishing = 29,
    InlineSkating = 30,
    RockClimbing = 31,
    Sailing = 32,
    IceSkating = 33,
    SkyDiving = 34,
    Snowshoeing = 35,
    Snowmobiling = 36,
    StandUpPaddleboarding = 37,
    Surfing = 38,
    Wakeboarding = 39,
    WaterSkiing = 40,
    Kayaking = 41,
    Rafting = 42,
    Windsurfing = 43,
    Kitesurfing = 44,
    Tactical = 45,
    Jumpmaster = 46,
    Boxing = 47,
    FloorClimbing = 48,
    #[doc = "All is for goals only to include all sports."]
    All = 254,
    Unknown,
}
impl Sport {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Sport::Generic),
            1 => Ok(Sport::Running),
            2 => Ok(Sport::Cycling),
            3 => Ok(Sport::Transition),
            4 => Ok(Sport::FitnessEquipment),
            5 => Ok(Sport::Swimming),
            6 => Ok(Sport::Basketball),
            7 => Ok(Sport::Soccer),
            8 => Ok(Sport::Tennis),
            9 => Ok(Sport::AmericanFootball),
            10 => Ok(Sport::Training),
            11 => Ok(Sport::Walking),
            12 => Ok(Sport::CrossCountrySkiing),
            13 => Ok(Sport::AlpineSkiing),
            14 => Ok(Sport::Snowboarding),
            15 => Ok(Sport::Rowing),
            16 => Ok(Sport::Mountaineering),
            17 => Ok(Sport::Hiking),
            18 => Ok(Sport::Multisport),
            19 => Ok(Sport::Paddling),
            20 => Ok(Sport::Flying),
            21 => Ok(Sport::EBiking),
            22 => Ok(Sport::Motorcycling),
            23 => Ok(Sport::Boating),
            24 => Ok(Sport::Driving),
            25 => Ok(Sport::Golf),
            26 => Ok(Sport::HangGliding),
            27 => Ok(Sport::HorsebackRiding),
            28 => Ok(Sport::Hunting),
            29 => Ok(Sport::Fishing),
            30 => Ok(Sport::InlineSkating),
            31 => Ok(Sport::RockClimbing),
            32 => Ok(Sport::Sailing),
            33 => Ok(Sport::IceSkating),
            34 => Ok(Sport::SkyDiving),
            35 => Ok(Sport::Snowshoeing),
            36 => Ok(Sport::Snowmobiling),
            37 => Ok(Sport::StandUpPaddleboarding),
            38 => Ok(Sport::Surfing),
            39 => Ok(Sport::Wakeboarding),
            40 => Ok(Sport::WaterSkiing),
            41 => Ok(Sport::Kayaking),
            42 => Ok(Sport::Rafting),
            43 => Ok(Sport::Windsurfing),
            44 => Ok(Sport::Kitesurfing),
            45 => Ok(Sport::Tactical),
            46 => Ok(Sport::Jumpmaster),
            47 => Ok(Sport::Boxing),
            48 => Ok(Sport::FloorClimbing),
            254 => Ok(Sport::All),
            _ => Ok(Sport::Unknown),
        }
    }
}
#[doc = "Bit field corresponding to sport enum type (1 << sport)."]
#[derive(Debug)]
pub enum SportBits0 {
    Generic = 1,
    Running = 2,
    Cycling = 4,
    #[doc = "Mulitsport transition"]
    Transition = 8,
    FitnessEquipment = 16,
    Swimming = 32,
    Basketball = 64,
    Soccer = 128,
    Unknown,
}
impl SportBits0 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(SportBits0::Generic),
            2 => Ok(SportBits0::Running),
            4 => Ok(SportBits0::Cycling),
            8 => Ok(SportBits0::Transition),
            16 => Ok(SportBits0::FitnessEquipment),
            32 => Ok(SportBits0::Swimming),
            64 => Ok(SportBits0::Basketball),
            128 => Ok(SportBits0::Soccer),
            _ => Ok(SportBits0::Unknown),
        }
    }
}
#[doc = "Bit field corresponding to sport enum type (1 << (sport-8))."]
#[derive(Debug)]
pub enum SportBits1 {
    Tennis = 1,
    AmericanFootball = 2,
    Training = 4,
    Walking = 8,
    CrossCountrySkiing = 16,
    AlpineSkiing = 32,
    Snowboarding = 64,
    Rowing = 128,
    Unknown,
}
impl SportBits1 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(SportBits1::Tennis),
            2 => Ok(SportBits1::AmericanFootball),
            4 => Ok(SportBits1::Training),
            8 => Ok(SportBits1::Walking),
            16 => Ok(SportBits1::CrossCountrySkiing),
            32 => Ok(SportBits1::AlpineSkiing),
            64 => Ok(SportBits1::Snowboarding),
            128 => Ok(SportBits1::Rowing),
            _ => Ok(SportBits1::Unknown),
        }
    }
}
#[doc = "Bit field corresponding to sport enum type (1 << (sport-16))."]
#[derive(Debug)]
pub enum SportBits2 {
    Mountaineering = 1,
    Hiking = 2,
    Multisport = 4,
    Paddling = 8,
    Flying = 16,
    EBiking = 32,
    Motorcycling = 64,
    Boating = 128,
    Unknown,
}
impl SportBits2 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(SportBits2::Mountaineering),
            2 => Ok(SportBits2::Hiking),
            4 => Ok(SportBits2::Multisport),
            8 => Ok(SportBits2::Paddling),
            16 => Ok(SportBits2::Flying),
            32 => Ok(SportBits2::EBiking),
            64 => Ok(SportBits2::Motorcycling),
            128 => Ok(SportBits2::Boating),
            _ => Ok(SportBits2::Unknown),
        }
    }
}
#[doc = "Bit field corresponding to sport enum type (1 << (sport-24))."]
#[derive(Debug)]
pub enum SportBits3 {
    Driving = 1,
    Golf = 2,
    HangGliding = 4,
    HorsebackRiding = 8,
    Hunting = 16,
    Fishing = 32,
    InlineSkating = 64,
    RockClimbing = 128,
    Unknown,
}
impl SportBits3 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(SportBits3::Driving),
            2 => Ok(SportBits3::Golf),
            4 => Ok(SportBits3::HangGliding),
            8 => Ok(SportBits3::HorsebackRiding),
            16 => Ok(SportBits3::Hunting),
            32 => Ok(SportBits3::Fishing),
            64 => Ok(SportBits3::InlineSkating),
            128 => Ok(SportBits3::RockClimbing),
            _ => Ok(SportBits3::Unknown),
        }
    }
}
#[doc = "Bit field corresponding to sport enum type (1 << (sport-32))."]
#[derive(Debug)]
pub enum SportBits4 {
    Sailing = 1,
    IceSkating = 2,
    SkyDiving = 4,
    Snowshoeing = 8,
    Snowmobiling = 16,
    StandUpPaddleboarding = 32,
    Surfing = 64,
    Wakeboarding = 128,
    Unknown,
}
impl SportBits4 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(SportBits4::Sailing),
            2 => Ok(SportBits4::IceSkating),
            4 => Ok(SportBits4::SkyDiving),
            8 => Ok(SportBits4::Snowshoeing),
            16 => Ok(SportBits4::Snowmobiling),
            32 => Ok(SportBits4::StandUpPaddleboarding),
            64 => Ok(SportBits4::Surfing),
            128 => Ok(SportBits4::Wakeboarding),
            _ => Ok(SportBits4::Unknown),
        }
    }
}
#[doc = "Bit field corresponding to sport enum type (1 << (sport-40))."]
#[derive(Debug)]
pub enum SportBits5 {
    WaterSkiing = 1,
    Kayaking = 2,
    Rafting = 4,
    Windsurfing = 8,
    Kitesurfing = 16,
    Tactical = 32,
    Jumpmaster = 64,
    Boxing = 128,
    Unknown,
}
impl SportBits5 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(SportBits5::WaterSkiing),
            2 => Ok(SportBits5::Kayaking),
            4 => Ok(SportBits5::Rafting),
            8 => Ok(SportBits5::Windsurfing),
            16 => Ok(SportBits5::Kitesurfing),
            32 => Ok(SportBits5::Tactical),
            64 => Ok(SportBits5::Jumpmaster),
            128 => Ok(SportBits5::Boxing),
            _ => Ok(SportBits5::Unknown),
        }
    }
}
#[doc = "Bit field corresponding to sport enum type (1 << (sport-48))."]
#[derive(Debug)]
pub enum SportBits6 {
    FloorClimbing = 1,
    Unknown,
}
impl SportBits6 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(SportBits6::FloorClimbing),
            _ => Ok(SportBits6::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SubSport {
    Generic = 0,
    #[doc = "Run/Fitness Equipment"]
    Treadmill = 1,
    #[doc = "Run"]
    Street = 2,
    #[doc = "Run"]
    Trail = 3,
    #[doc = "Run"]
    Track = 4,
    #[doc = "Cycling"]
    Spin = 5,
    #[doc = "Cycling/Fitness Equipment"]
    IndoorCycling = 6,
    #[doc = "Cycling"]
    Road = 7,
    #[doc = "Cycling"]
    Mountain = 8,
    #[doc = "Cycling"]
    Downhill = 9,
    #[doc = "Cycling"]
    Recumbent = 10,
    #[doc = "Cycling"]
    Cyclocross = 11,
    #[doc = "Cycling"]
    HandCycling = 12,
    #[doc = "Cycling"]
    TrackCycling = 13,
    #[doc = "Fitness Equipment"]
    IndoorRowing = 14,
    #[doc = "Fitness Equipment"]
    Elliptical = 15,
    #[doc = "Fitness Equipment"]
    StairClimbing = 16,
    #[doc = "Swimming"]
    LapSwimming = 17,
    #[doc = "Swimming"]
    OpenWater = 18,
    #[doc = "Training"]
    FlexibilityTraining = 19,
    #[doc = "Training"]
    StrengthTraining = 20,
    #[doc = "Tennis"]
    WarmUp = 21,
    #[doc = "Tennis"]
    Match = 22,
    #[doc = "Tennis"]
    Exercise = 23,
    Challenge = 24,
    #[doc = "Fitness Equipment"]
    IndoorSkiing = 25,
    #[doc = "Training"]
    CardioTraining = 26,
    #[doc = "Walking/Fitness Equipment"]
    IndoorWalking = 27,
    #[doc = "E-Biking"]
    EBikeFitness = 28,
    #[doc = "Cycling"]
    Bmx = 29,
    #[doc = "Walking"]
    CasualWalking = 30,
    #[doc = "Walking"]
    SpeedWalking = 31,
    #[doc = "Transition"]
    BikeToRunTransition = 32,
    #[doc = "Transition"]
    RunToBikeTransition = 33,
    #[doc = "Transition"]
    SwimToBikeTransition = 34,
    #[doc = "Motorcycling"]
    Atv = 35,
    #[doc = "Motorcycling"]
    Motocross = 36,
    #[doc = "Alpine Skiing/Snowboarding"]
    Backcountry = 37,
    #[doc = "Alpine Skiing/Snowboarding"]
    Resort = 38,
    #[doc = "Flying"]
    RcDrone = 39,
    #[doc = "Flying"]
    Wingsuit = 40,
    #[doc = "Kayaking/Rafting"]
    Whitewater = 41,
    #[doc = "Cross Country Skiing"]
    SkateSkiing = 42,
    #[doc = "Training"]
    Yoga = 43,
    #[doc = "Training"]
    Pilates = 44,
    #[doc = "Run"]
    IndoorRunning = 45,
    #[doc = "Cycling"]
    GravelCycling = 46,
    #[doc = "Cycling"]
    EBikeMountain = 47,
    #[doc = "Cycling"]
    Commuting = 48,
    #[doc = "Cycling"]
    MixedSurface = 49,
    Navigate = 50,
    TrackMe = 51,
    Map = 52,
    #[doc = "Diving"]
    SingleGasDiving = 53,
    #[doc = "Diving"]
    MultiGasDiving = 54,
    #[doc = "Diving"]
    GaugeDiving = 55,
    #[doc = "Diving"]
    ApneaDiving = 56,
    #[doc = "Diving"]
    ApneaHunting = 57,
    VirtualActivity = 58,
    #[doc = "Used for events where participants run, crawl through mud, climb \
             over walls, etc."]
    Obstacle = 59,
    All = 254,
    Unknown,
}
impl SubSport {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SubSport::Generic),
            1 => Ok(SubSport::Treadmill),
            2 => Ok(SubSport::Street),
            3 => Ok(SubSport::Trail),
            4 => Ok(SubSport::Track),
            5 => Ok(SubSport::Spin),
            6 => Ok(SubSport::IndoorCycling),
            7 => Ok(SubSport::Road),
            8 => Ok(SubSport::Mountain),
            9 => Ok(SubSport::Downhill),
            10 => Ok(SubSport::Recumbent),
            11 => Ok(SubSport::Cyclocross),
            12 => Ok(SubSport::HandCycling),
            13 => Ok(SubSport::TrackCycling),
            14 => Ok(SubSport::IndoorRowing),
            15 => Ok(SubSport::Elliptical),
            16 => Ok(SubSport::StairClimbing),
            17 => Ok(SubSport::LapSwimming),
            18 => Ok(SubSport::OpenWater),
            19 => Ok(SubSport::FlexibilityTraining),
            20 => Ok(SubSport::StrengthTraining),
            21 => Ok(SubSport::WarmUp),
            22 => Ok(SubSport::Match),
            23 => Ok(SubSport::Exercise),
            24 => Ok(SubSport::Challenge),
            25 => Ok(SubSport::IndoorSkiing),
            26 => Ok(SubSport::CardioTraining),
            27 => Ok(SubSport::IndoorWalking),
            28 => Ok(SubSport::EBikeFitness),
            29 => Ok(SubSport::Bmx),
            30 => Ok(SubSport::CasualWalking),
            31 => Ok(SubSport::SpeedWalking),
            32 => Ok(SubSport::BikeToRunTransition),
            33 => Ok(SubSport::RunToBikeTransition),
            34 => Ok(SubSport::SwimToBikeTransition),
            35 => Ok(SubSport::Atv),
            36 => Ok(SubSport::Motocross),
            37 => Ok(SubSport::Backcountry),
            38 => Ok(SubSport::Resort),
            39 => Ok(SubSport::RcDrone),
            40 => Ok(SubSport::Wingsuit),
            41 => Ok(SubSport::Whitewater),
            42 => Ok(SubSport::SkateSkiing),
            43 => Ok(SubSport::Yoga),
            44 => Ok(SubSport::Pilates),
            45 => Ok(SubSport::IndoorRunning),
            46 => Ok(SubSport::GravelCycling),
            47 => Ok(SubSport::EBikeMountain),
            48 => Ok(SubSport::Commuting),
            49 => Ok(SubSport::MixedSurface),
            50 => Ok(SubSport::Navigate),
            51 => Ok(SubSport::TrackMe),
            52 => Ok(SubSport::Map),
            53 => Ok(SubSport::SingleGasDiving),
            54 => Ok(SubSport::MultiGasDiving),
            55 => Ok(SubSport::GaugeDiving),
            56 => Ok(SubSport::ApneaDiving),
            57 => Ok(SubSport::ApneaHunting),
            58 => Ok(SubSport::VirtualActivity),
            59 => Ok(SubSport::Obstacle),
            254 => Ok(SubSport::All),
            _ => Ok(SubSport::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SportEvent {
    Uncategorized = 0,
    Geocaching = 1,
    Fitness = 2,
    Recreation = 3,
    Race = 4,
    SpecialEvent = 5,
    Training = 6,
    Transportation = 7,
    Touring = 8,
    Unknown,
}
impl SportEvent {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SportEvent::Uncategorized),
            1 => Ok(SportEvent::Geocaching),
            2 => Ok(SportEvent::Fitness),
            3 => Ok(SportEvent::Recreation),
            4 => Ok(SportEvent::Race),
            5 => Ok(SportEvent::SpecialEvent),
            6 => Ok(SportEvent::Training),
            7 => Ok(SportEvent::Transportation),
            8 => Ok(SportEvent::Touring),
            _ => Ok(SportEvent::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Activity {
    Manual = 0,
    AutoMultiSport = 1,
    Unknown,
}
impl Activity {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Activity::Manual),
            1 => Ok(Activity::AutoMultiSport),
            _ => Ok(Activity::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Intensity {
    Active = 0,
    Rest = 1,
    Warmup = 2,
    Cooldown = 3,
    Unknown,
}
impl Intensity {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Intensity::Active),
            1 => Ok(Intensity::Rest),
            2 => Ok(Intensity::Warmup),
            3 => Ok(Intensity::Cooldown),
            _ => Ok(Intensity::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SessionTrigger {
    ActivityEnd = 0,
    #[doc = "User changed sport."]
    Manual = 1,
    #[doc = "Auto multi-sport feature is enabled and user pressed lap button \
             to advance session."]
    AutoMultiSport = 2,
    #[doc = "Auto sport change caused by user linking to fitness equipment."]
    FitnessEquipment = 3,
    Unknown,
}
impl SessionTrigger {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SessionTrigger::ActivityEnd),
            1 => Ok(SessionTrigger::Manual),
            2 => Ok(SessionTrigger::AutoMultiSport),
            3 => Ok(SessionTrigger::FitnessEquipment),
            _ => Ok(SessionTrigger::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum AutolapTrigger {
    Time = 0,
    Distance = 1,
    PositionStart = 2,
    PositionLap = 3,
    PositionWaypoint = 4,
    PositionMarked = 5,
    Off = 6,
    Unknown,
}
impl AutolapTrigger {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(AutolapTrigger::Time),
            1 => Ok(AutolapTrigger::Distance),
            2 => Ok(AutolapTrigger::PositionStart),
            3 => Ok(AutolapTrigger::PositionLap),
            4 => Ok(AutolapTrigger::PositionWaypoint),
            5 => Ok(AutolapTrigger::PositionMarked),
            6 => Ok(AutolapTrigger::Off),
            _ => Ok(AutolapTrigger::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LapTrigger {
    Manual = 0,
    Time = 1,
    Distance = 2,
    PositionStart = 3,
    PositionLap = 4,
    PositionWaypoint = 5,
    PositionMarked = 6,
    SessionEnd = 7,
    FitnessEquipment = 8,
    Unknown,
}
impl LapTrigger {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(LapTrigger::Manual),
            1 => Ok(LapTrigger::Time),
            2 => Ok(LapTrigger::Distance),
            3 => Ok(LapTrigger::PositionStart),
            4 => Ok(LapTrigger::PositionLap),
            5 => Ok(LapTrigger::PositionWaypoint),
            6 => Ok(LapTrigger::PositionMarked),
            7 => Ok(LapTrigger::SessionEnd),
            8 => Ok(LapTrigger::FitnessEquipment),
            _ => Ok(LapTrigger::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum TimeMode {
    Hour12 = 0,
    #[doc = "Does not use a leading zero and has a colon"]
    Hour24 = 1,
    #[doc = "Uses a leading zero and does not have a colon"]
    Military = 2,
    Hour12WithSeconds = 3,
    Hour24WithSeconds = 4,
    Utc = 5,
    Unknown,
}
impl TimeMode {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(TimeMode::Hour12),
            1 => Ok(TimeMode::Hour24),
            2 => Ok(TimeMode::Military),
            3 => Ok(TimeMode::Hour12WithSeconds),
            4 => Ok(TimeMode::Hour24WithSeconds),
            5 => Ok(TimeMode::Utc),
            _ => Ok(TimeMode::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum BacklightMode {
    Off = 0,
    Manual = 1,
    KeyAndMessages = 2,
    AutoBrightness = 3,
    SmartNotifications = 4,
    KeyAndMessagesNight = 5,
    KeyAndMessagesAndSmartNotifications = 6,
    Unknown,
}
impl BacklightMode {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(BacklightMode::Off),
            1 => Ok(BacklightMode::Manual),
            2 => Ok(BacklightMode::KeyAndMessages),
            3 => Ok(BacklightMode::AutoBrightness),
            4 => Ok(BacklightMode::SmartNotifications),
            5 => Ok(BacklightMode::KeyAndMessagesNight),
            6 => Ok(BacklightMode::KeyAndMessagesAndSmartNotifications),
            _ => Ok(BacklightMode::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DateMode {
    DayMonth = 0,
    MonthDay = 1,
    Unknown,
}
impl DateMode {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DateMode::DayMonth),
            1 => Ok(DateMode::MonthDay),
            _ => Ok(DateMode::Unknown),
        }
    }
}
#[doc = "Timeout in seconds."]
#[derive(Debug)]
pub enum BacklightTimeout {
    #[doc = "Backlight stays on forever."]
    Infinite = 0,
    Unknown,
}
impl BacklightTimeout {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(BacklightTimeout::Infinite),
            _ => Ok(BacklightTimeout::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Event {
    #[doc = "Group 0.  Start / stop_all"]
    Timer = 0,
    #[doc = "start / stop"]
    Workout = 3,
    #[doc = "Start at beginning of workout.  Stop at end of each step."]
    WorkoutStep = 4,
    #[doc = "stop_all group 0"]
    PowerDown = 5,
    #[doc = "stop_all group 0"]
    PowerUp = 6,
    #[doc = "start / stop group 0"]
    OffCourse = 7,
    #[doc = "Stop at end of each session."]
    Session = 8,
    #[doc = "Stop at end of each lap."]
    Lap = 9,
    #[doc = "marker"]
    CoursePoint = 10,
    #[doc = "marker"]
    Battery = 11,
    #[doc = "Group 1. Start at beginning of activity if VP enabled, when VP \
             pace is changed during activity or VP enabled mid activity.  \
             stop_disable when VP disabled."]
    VirtualPartnerPace = 12,
    #[doc = "Group 0.  Start / stop when in alert condition."]
    HrHighAlert = 13,
    #[doc = "Group 0.  Start / stop when in alert condition."]
    HrLowAlert = 14,
    #[doc = "Group 0.  Start / stop when in alert condition."]
    SpeedHighAlert = 15,
    #[doc = "Group 0.  Start / stop when in alert condition."]
    SpeedLowAlert = 16,
    #[doc = "Group 0.  Start / stop when in alert condition."]
    CadHighAlert = 17,
    #[doc = "Group 0.  Start / stop when in alert condition."]
    CadLowAlert = 18,
    #[doc = "Group 0.  Start / stop when in alert condition."]
    PowerHighAlert = 19,
    #[doc = "Group 0.  Start / stop when in alert condition."]
    PowerLowAlert = 20,
    #[doc = "marker"]
    RecoveryHr = 21,
    #[doc = "marker"]
    BatteryLow = 22,
    #[doc = "Group 1.  Start if enabled mid activity (not required at start \
             of activity). Stop when duration is reached.  stop_disable if \
             disabled."]
    TimeDurationAlert = 23,
    #[doc = "Group 1.  Start if enabled mid activity (not required at start \
             of activity). Stop when duration is reached.  stop_disable if \
             disabled."]
    DistanceDurationAlert = 24,
    #[doc = "Group 1.  Start if enabled mid activity (not required at start \
             of activity). Stop when duration is reached.  stop_disable if \
             disabled."]
    CalorieDurationAlert = 25,
    #[doc = "Group 1..  Stop at end of activity."]
    Activity = 26,
    #[doc = "marker"]
    FitnessEquipment = 27,
    #[doc = "Stop at end of each length."]
    Length = 28,
    #[doc = "marker"]
    UserMarker = 32,
    #[doc = "marker"]
    SportPoint = 33,
    #[doc = "start/stop/marker"]
    Calibration = 36,
    #[doc = "marker"]
    FrontGearChange = 42,
    #[doc = "marker"]
    RearGearChange = 43,
    #[doc = "marker"]
    RiderPositionChange = 44,
    #[doc = "Group 0.  Start / stop when in alert condition."]
    ElevHighAlert = 45,
    #[doc = "Group 0.  Start / stop when in alert condition."]
    ElevLowAlert = 46,
    #[doc = "marker"]
    CommTimeout = 47,
    Unknown,
}
impl Event {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Event::Timer),
            3 => Ok(Event::Workout),
            4 => Ok(Event::WorkoutStep),
            5 => Ok(Event::PowerDown),
            6 => Ok(Event::PowerUp),
            7 => Ok(Event::OffCourse),
            8 => Ok(Event::Session),
            9 => Ok(Event::Lap),
            10 => Ok(Event::CoursePoint),
            11 => Ok(Event::Battery),
            12 => Ok(Event::VirtualPartnerPace),
            13 => Ok(Event::HrHighAlert),
            14 => Ok(Event::HrLowAlert),
            15 => Ok(Event::SpeedHighAlert),
            16 => Ok(Event::SpeedLowAlert),
            17 => Ok(Event::CadHighAlert),
            18 => Ok(Event::CadLowAlert),
            19 => Ok(Event::PowerHighAlert),
            20 => Ok(Event::PowerLowAlert),
            21 => Ok(Event::RecoveryHr),
            22 => Ok(Event::BatteryLow),
            23 => Ok(Event::TimeDurationAlert),
            24 => Ok(Event::DistanceDurationAlert),
            25 => Ok(Event::CalorieDurationAlert),
            26 => Ok(Event::Activity),
            27 => Ok(Event::FitnessEquipment),
            28 => Ok(Event::Length),
            32 => Ok(Event::UserMarker),
            33 => Ok(Event::SportPoint),
            36 => Ok(Event::Calibration),
            42 => Ok(Event::FrontGearChange),
            43 => Ok(Event::RearGearChange),
            44 => Ok(Event::RiderPositionChange),
            45 => Ok(Event::ElevHighAlert),
            46 => Ok(Event::ElevLowAlert),
            47 => Ok(Event::CommTimeout),
            _ => Ok(Event::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum EventType {
    Start = 0,
    Stop = 1,
    ConsecutiveDepreciated = 2,
    Marker = 3,
    StopAll = 4,
    BeginDepreciated = 5,
    EndDepreciated = 6,
    EndAllDepreciated = 7,
    StopDisable = 8,
    StopDisableAll = 9,
    Unknown,
}
impl EventType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(EventType::Start),
            1 => Ok(EventType::Stop),
            2 => Ok(EventType::ConsecutiveDepreciated),
            3 => Ok(EventType::Marker),
            4 => Ok(EventType::StopAll),
            5 => Ok(EventType::BeginDepreciated),
            6 => Ok(EventType::EndDepreciated),
            7 => Ok(EventType::EndAllDepreciated),
            8 => Ok(EventType::StopDisable),
            9 => Ok(EventType::StopDisableAll),
            _ => Ok(EventType::Unknown),
        }
    }
}
#[doc = "timer event data"]
#[derive(Debug)]
pub enum TimerTrigger {
    Manual = 0,
    Auto = 1,
    FitnessEquipment = 2,
    Unknown,
}
impl TimerTrigger {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(TimerTrigger::Manual),
            1 => Ok(TimerTrigger::Auto),
            2 => Ok(TimerTrigger::FitnessEquipment),
            _ => Ok(TimerTrigger::Unknown),
        }
    }
}
#[doc = "fitness equipment event data"]
#[derive(Debug)]
pub enum FitnessEquipmentState {
    Ready = 0,
    InUse = 1,
    Paused = 2,
    Unknown,
}
impl FitnessEquipmentState {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(FitnessEquipmentState::Ready),
            1 => Ok(FitnessEquipmentState::InUse),
            2 => Ok(FitnessEquipmentState::Paused),
            3 => Ok(FitnessEquipmentState::Unknown),
            _ => Ok(FitnessEquipmentState::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Tone {
    Off = 0,
    Tone = 1,
    Vibrate = 2,
    ToneAndVibrate = 3,
    Unknown,
}
impl Tone {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Tone::Off),
            1 => Ok(Tone::Tone),
            2 => Ok(Tone::Vibrate),
            3 => Ok(Tone::ToneAndVibrate),
            _ => Ok(Tone::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Autoscroll {
    None = 0,
    Slow = 1,
    Medium = 2,
    Fast = 3,
    Unknown,
}
impl Autoscroll {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Autoscroll::None),
            1 => Ok(Autoscroll::Slow),
            2 => Ok(Autoscroll::Medium),
            3 => Ok(Autoscroll::Fast),
            _ => Ok(Autoscroll::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ActivityClass {
    #[doc = "0 to 100"]
    Level = 127,
    LevelMax = 100,
    Athlete = 128,
    Unknown,
}
impl ActivityClass {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            127 => Ok(ActivityClass::Level),
            100 => Ok(ActivityClass::LevelMax),
            128 => Ok(ActivityClass::Athlete),
            _ => Ok(ActivityClass::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum HrZoneCalc {
    Custom = 0,
    PercentMaxHr = 1,
    PercentHrr = 2,
    Unknown,
}
impl HrZoneCalc {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(HrZoneCalc::Custom),
            1 => Ok(HrZoneCalc::PercentMaxHr),
            2 => Ok(HrZoneCalc::PercentHrr),
            _ => Ok(HrZoneCalc::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum PwrZoneCalc {
    Custom = 0,
    PercentFtp = 1,
    Unknown,
}
impl PwrZoneCalc {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(PwrZoneCalc::Custom),
            1 => Ok(PwrZoneCalc::PercentFtp),
            _ => Ok(PwrZoneCalc::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum WktStepDuration {
    Time = 0,
    Distance = 1,
    HrLessThan = 2,
    HrGreaterThan = 3,
    Calories = 4,
    Open = 5,
    RepeatUntilStepsCmplt = 6,
    RepeatUntilTime = 7,
    RepeatUntilDistance = 8,
    RepeatUntilCalories = 9,
    RepeatUntilHrLessThan = 10,
    RepeatUntilHrGreaterThan = 11,
    RepeatUntilPowerLessThan = 12,
    RepeatUntilPowerGreaterThan = 13,
    PowerLessThan = 14,
    PowerGreaterThan = 15,
    TrainingPeaksTss = 16,
    RepeatUntilPowerLastLapLessThan = 17,
    RepeatUntilMaxPowerLastLapLessThan = 18,
    Power3SLessThan = 19,
    Power10SLessThan = 20,
    Power30SLessThan = 21,
    Power3SGreaterThan = 22,
    Power10SGreaterThan = 23,
    Power30SGreaterThan = 24,
    PowerLapLessThan = 25,
    PowerLapGreaterThan = 26,
    RepeatUntilTrainingPeaksTss = 27,
    RepetitionTime = 28,
    Reps = 29,
    Unknown,
}
impl WktStepDuration {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(WktStepDuration::Time),
            1 => Ok(WktStepDuration::Distance),
            2 => Ok(WktStepDuration::HrLessThan),
            3 => Ok(WktStepDuration::HrGreaterThan),
            4 => Ok(WktStepDuration::Calories),
            5 => Ok(WktStepDuration::Open),
            6 => Ok(WktStepDuration::RepeatUntilStepsCmplt),
            7 => Ok(WktStepDuration::RepeatUntilTime),
            8 => Ok(WktStepDuration::RepeatUntilDistance),
            9 => Ok(WktStepDuration::RepeatUntilCalories),
            10 => Ok(WktStepDuration::RepeatUntilHrLessThan),
            11 => Ok(WktStepDuration::RepeatUntilHrGreaterThan),
            12 => Ok(WktStepDuration::RepeatUntilPowerLessThan),
            13 => Ok(WktStepDuration::RepeatUntilPowerGreaterThan),
            14 => Ok(WktStepDuration::PowerLessThan),
            15 => Ok(WktStepDuration::PowerGreaterThan),
            16 => Ok(WktStepDuration::TrainingPeaksTss),
            17 => Ok(WktStepDuration::RepeatUntilPowerLastLapLessThan),
            18 => Ok(WktStepDuration::RepeatUntilMaxPowerLastLapLessThan),
            19 => Ok(WktStepDuration::Power3SLessThan),
            20 => Ok(WktStepDuration::Power10SLessThan),
            21 => Ok(WktStepDuration::Power30SLessThan),
            22 => Ok(WktStepDuration::Power3SGreaterThan),
            23 => Ok(WktStepDuration::Power10SGreaterThan),
            24 => Ok(WktStepDuration::Power30SGreaterThan),
            25 => Ok(WktStepDuration::PowerLapLessThan),
            26 => Ok(WktStepDuration::PowerLapGreaterThan),
            27 => Ok(WktStepDuration::RepeatUntilTrainingPeaksTss),
            28 => Ok(WktStepDuration::RepetitionTime),
            29 => Ok(WktStepDuration::Reps),
            _ => Ok(WktStepDuration::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum WktStepTarget {
    Speed = 0,
    HeartRate = 1,
    Open = 2,
    Cadence = 3,
    Power = 4,
    Grade = 5,
    Resistance = 6,
    Power3S = 7,
    Power10S = 8,
    Power30S = 9,
    PowerLap = 10,
    SwimStroke = 11,
    SpeedLap = 12,
    HeartRateLap = 13,
    Unknown,
}
impl WktStepTarget {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(WktStepTarget::Speed),
            1 => Ok(WktStepTarget::HeartRate),
            2 => Ok(WktStepTarget::Open),
            3 => Ok(WktStepTarget::Cadence),
            4 => Ok(WktStepTarget::Power),
            5 => Ok(WktStepTarget::Grade),
            6 => Ok(WktStepTarget::Resistance),
            7 => Ok(WktStepTarget::Power3S),
            8 => Ok(WktStepTarget::Power10S),
            9 => Ok(WktStepTarget::Power30S),
            10 => Ok(WktStepTarget::PowerLap),
            11 => Ok(WktStepTarget::SwimStroke),
            12 => Ok(WktStepTarget::SpeedLap),
            13 => Ok(WktStepTarget::HeartRateLap),
            _ => Ok(WktStepTarget::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Goal {
    Time = 0,
    Distance = 1,
    Calories = 2,
    Frequency = 3,
    Steps = 4,
    Ascent = 5,
    ActiveMinutes = 6,
    Unknown,
}
impl Goal {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Goal::Time),
            1 => Ok(Goal::Distance),
            2 => Ok(Goal::Calories),
            3 => Ok(Goal::Frequency),
            4 => Ok(Goal::Steps),
            5 => Ok(Goal::Ascent),
            6 => Ok(Goal::ActiveMinutes),
            _ => Ok(Goal::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum GoalRecurrence {
    Off = 0,
    Daily = 1,
    Weekly = 2,
    Monthly = 3,
    Yearly = 4,
    Custom = 5,
    Unknown,
}
impl GoalRecurrence {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(GoalRecurrence::Off),
            1 => Ok(GoalRecurrence::Daily),
            2 => Ok(GoalRecurrence::Weekly),
            3 => Ok(GoalRecurrence::Monthly),
            4 => Ok(GoalRecurrence::Yearly),
            5 => Ok(GoalRecurrence::Custom),
            _ => Ok(GoalRecurrence::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum GoalSource {
    #[doc = "Device generated"]
    Auto = 0,
    #[doc = "Social network sourced goal"]
    Community = 1,
    #[doc = "Manually generated"]
    User = 2,
    Unknown,
}
impl GoalSource {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(GoalSource::Auto),
            1 => Ok(GoalSource::Community),
            2 => Ok(GoalSource::User),
            _ => Ok(GoalSource::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Schedule {
    Workout = 0,
    Course = 1,
    Unknown,
}
impl Schedule {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Schedule::Workout),
            1 => Ok(Schedule::Course),
            _ => Ok(Schedule::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum CoursePoint {
    Generic = 0,
    Summit = 1,
    Valley = 2,
    Water = 3,
    Food = 4,
    Danger = 5,
    Left = 6,
    Right = 7,
    Straight = 8,
    FirstAid = 9,
    FourthCategory = 10,
    ThirdCategory = 11,
    SecondCategory = 12,
    FirstCategory = 13,
    HorsCategory = 14,
    Sprint = 15,
    LeftFork = 16,
    RightFork = 17,
    MiddleFork = 18,
    SlightLeft = 19,
    SharpLeft = 20,
    SlightRight = 21,
    SharpRight = 22,
    UTurn = 23,
    SegmentStart = 24,
    SegmentEnd = 25,
    Unknown,
}
impl CoursePoint {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(CoursePoint::Generic),
            1 => Ok(CoursePoint::Summit),
            2 => Ok(CoursePoint::Valley),
            3 => Ok(CoursePoint::Water),
            4 => Ok(CoursePoint::Food),
            5 => Ok(CoursePoint::Danger),
            6 => Ok(CoursePoint::Left),
            7 => Ok(CoursePoint::Right),
            8 => Ok(CoursePoint::Straight),
            9 => Ok(CoursePoint::FirstAid),
            10 => Ok(CoursePoint::FourthCategory),
            11 => Ok(CoursePoint::ThirdCategory),
            12 => Ok(CoursePoint::SecondCategory),
            13 => Ok(CoursePoint::FirstCategory),
            14 => Ok(CoursePoint::HorsCategory),
            15 => Ok(CoursePoint::Sprint),
            16 => Ok(CoursePoint::LeftFork),
            17 => Ok(CoursePoint::RightFork),
            18 => Ok(CoursePoint::MiddleFork),
            19 => Ok(CoursePoint::SlightLeft),
            20 => Ok(CoursePoint::SharpLeft),
            21 => Ok(CoursePoint::SlightRight),
            22 => Ok(CoursePoint::SharpRight),
            23 => Ok(CoursePoint::UTurn),
            24 => Ok(CoursePoint::SegmentStart),
            25 => Ok(CoursePoint::SegmentEnd),
            _ => Ok(CoursePoint::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Manufacturer {
    Garmin = 1,
    #[doc = "Do not use.  Used by FR405 for ANTFS man id."]
    GarminFr405Antfs = 2,
    Zephyr = 3,
    Dayton = 4,
    Idt = 5,
    Srm = 6,
    Quarq = 7,
    Ibike = 8,
    Saris = 9,
    SparkHk = 10,
    Tanita = 11,
    Echowell = 12,
    DynastreamOem = 13,
    Nautilus = 14,
    Dynastream = 15,
    Timex = 16,
    Metrigear = 17,
    Xelic = 18,
    Beurer = 19,
    Cardiosport = 20,
    AAndD = 21,
    Hmm = 22,
    Suunto = 23,
    ThitaElektronik = 24,
    Gpulse = 25,
    CleanMobile = 26,
    PedalBrain = 27,
    Peaksware = 28,
    Saxonar = 29,
    LemondFitness = 30,
    Dexcom = 31,
    WahooFitness = 32,
    OctaneFitness = 33,
    Archinoetics = 34,
    TheHurtBox = 35,
    CitizenSystems = 36,
    Magellan = 37,
    Osynce = 38,
    Holux = 39,
    Concept2 = 40,
    OneGiantLeap = 42,
    AceSensor = 43,
    BrimBrothers = 44,
    Xplova = 45,
    PerceptionDigital = 46,
    Bf1Systems = 47,
    Pioneer = 48,
    Spantec = 49,
    Metalogics = 50,
    Fouriiiis = 51,
    SeikoEpson = 52,
    SeikoEpsonOem = 53,
    IforPowell = 54,
    MaxwellGuider = 55,
    StarTrac = 56,
    Breakaway = 57,
    AlatechTechnologyLtd = 58,
    MioTechnologyEurope = 59,
    Rotor = 60,
    Geonaute = 61,
    IdBike = 62,
    Specialized = 63,
    Wtek = 64,
    PhysicalEnterprises = 65,
    NorthPoleEngineering = 66,
    Bkool = 67,
    Cateye = 68,
    StagesCycling = 69,
    Sigmasport = 70,
    Tomtom = 71,
    Peripedal = 72,
    Wattbike = 73,
    Moxy = 76,
    Ciclosport = 77,
    Powerbahn = 78,
    AcornProjectsAps = 79,
    Lifebeam = 80,
    Bontrager = 81,
    Wellgo = 82,
    Scosche = 83,
    Magura = 84,
    Woodway = 85,
    Elite = 86,
    NielsenKellerman = 87,
    DkCity = 88,
    Tacx = 89,
    DirectionTechnology = 90,
    Magtonic = 91,
    Onepartcarbon = 92,
    InsideRideTechnologies = 93,
    SoundOfMotion = 94,
    Stryd = 95,
    #[doc = "Indoorcycling Group"]
    Icg = 96,
    MiPulse = 97,
    BsxAthletics = 98,
    Look = 99,
    CampagnoloSrl = 100,
    BodyBikeSmart = 101,
    Praxisworks = 102,
    #[doc = "Limits Technology Ltd."]
    LimitsTechnology = 103,
    #[doc = "TopAction Technology Inc."]
    TopactionTechnology = 104,
    Cosinuss = 105,
    Fitcare = 106,
    Magene = 107,
    GiantManufacturingCo = 108,
    #[doc = "Tigrasport"]
    Tigrasport = 109,
    Salutron = 110,
    Technogym = 111,
    BrytonSensors = 112,
    LatitudeLimited = 113,
    SoaringTechnology = 114,
    Igpsport = 115,
    Thinkrider = 116,
    GopherSport = 117,
    Waterrower = 118,
    Orangetheory = 119,
    Inpeak = 120,
    Kinetic = 121,
    JohnsonHealthTech = 122,
    PolarElectro = 123,
    Seesense = 124,
    Development = 255,
    Healthandlife = 257,
    Lezyne = 258,
    ScribeLabs = 259,
    Zwift = 260,
    Watteam = 261,
    Recon = 262,
    FaveroElectronics = 263,
    Dynovelo = 264,
    Strava = 265,
    #[doc = "Amer Sports"]
    Precor = 266,
    Bryton = 267,
    Sram = 268,
    #[doc = "MiTAC Global Corporation (Mio Technology)"]
    Navman = 269,
    #[doc = "COBI GmbH"]
    Cobi = 270,
    Spivi = 271,
    MioMagellan = 272,
    Evesports = 273,
    SensitivusGauge = 274,
    Podoon = 275,
    LifeTimeFitness = 276,
    #[doc = "Falco eMotors Inc."]
    FalcoEMotors = 277,
    Minoura = 278,
    Cycliq = 279,
    Luxottica = 280,
    TrainerRoad = 281,
    TheSufferfest = 282,
    Fullspeedahead = 283,
    Virtualtraining = 284,
    Feedbacksports = 285,
    Omata = 286,
    Vdo = 287,
    Magneticdays = 288,
    Hammerhead = 289,
    KineticByKurt = 290,
    Shapelog = 291,
    Dabuziduo = 292,
    Jetblack = 293,
    Actigraphcorp = 5759,
    Unknown,
}
impl Manufacturer {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(Manufacturer::Garmin),
            2 => Ok(Manufacturer::GarminFr405Antfs),
            3 => Ok(Manufacturer::Zephyr),
            4 => Ok(Manufacturer::Dayton),
            5 => Ok(Manufacturer::Idt),
            6 => Ok(Manufacturer::Srm),
            7 => Ok(Manufacturer::Quarq),
            8 => Ok(Manufacturer::Ibike),
            9 => Ok(Manufacturer::Saris),
            10 => Ok(Manufacturer::SparkHk),
            11 => Ok(Manufacturer::Tanita),
            12 => Ok(Manufacturer::Echowell),
            13 => Ok(Manufacturer::DynastreamOem),
            14 => Ok(Manufacturer::Nautilus),
            15 => Ok(Manufacturer::Dynastream),
            16 => Ok(Manufacturer::Timex),
            17 => Ok(Manufacturer::Metrigear),
            18 => Ok(Manufacturer::Xelic),
            19 => Ok(Manufacturer::Beurer),
            20 => Ok(Manufacturer::Cardiosport),
            21 => Ok(Manufacturer::AAndD),
            22 => Ok(Manufacturer::Hmm),
            23 => Ok(Manufacturer::Suunto),
            24 => Ok(Manufacturer::ThitaElektronik),
            25 => Ok(Manufacturer::Gpulse),
            26 => Ok(Manufacturer::CleanMobile),
            27 => Ok(Manufacturer::PedalBrain),
            28 => Ok(Manufacturer::Peaksware),
            29 => Ok(Manufacturer::Saxonar),
            30 => Ok(Manufacturer::LemondFitness),
            31 => Ok(Manufacturer::Dexcom),
            32 => Ok(Manufacturer::WahooFitness),
            33 => Ok(Manufacturer::OctaneFitness),
            34 => Ok(Manufacturer::Archinoetics),
            35 => Ok(Manufacturer::TheHurtBox),
            36 => Ok(Manufacturer::CitizenSystems),
            37 => Ok(Manufacturer::Magellan),
            38 => Ok(Manufacturer::Osynce),
            39 => Ok(Manufacturer::Holux),
            40 => Ok(Manufacturer::Concept2),
            42 => Ok(Manufacturer::OneGiantLeap),
            43 => Ok(Manufacturer::AceSensor),
            44 => Ok(Manufacturer::BrimBrothers),
            45 => Ok(Manufacturer::Xplova),
            46 => Ok(Manufacturer::PerceptionDigital),
            47 => Ok(Manufacturer::Bf1Systems),
            48 => Ok(Manufacturer::Pioneer),
            49 => Ok(Manufacturer::Spantec),
            50 => Ok(Manufacturer::Metalogics),
            51 => Ok(Manufacturer::Fouriiiis),
            52 => Ok(Manufacturer::SeikoEpson),
            53 => Ok(Manufacturer::SeikoEpsonOem),
            54 => Ok(Manufacturer::IforPowell),
            55 => Ok(Manufacturer::MaxwellGuider),
            56 => Ok(Manufacturer::StarTrac),
            57 => Ok(Manufacturer::Breakaway),
            58 => Ok(Manufacturer::AlatechTechnologyLtd),
            59 => Ok(Manufacturer::MioTechnologyEurope),
            60 => Ok(Manufacturer::Rotor),
            61 => Ok(Manufacturer::Geonaute),
            62 => Ok(Manufacturer::IdBike),
            63 => Ok(Manufacturer::Specialized),
            64 => Ok(Manufacturer::Wtek),
            65 => Ok(Manufacturer::PhysicalEnterprises),
            66 => Ok(Manufacturer::NorthPoleEngineering),
            67 => Ok(Manufacturer::Bkool),
            68 => Ok(Manufacturer::Cateye),
            69 => Ok(Manufacturer::StagesCycling),
            70 => Ok(Manufacturer::Sigmasport),
            71 => Ok(Manufacturer::Tomtom),
            72 => Ok(Manufacturer::Peripedal),
            73 => Ok(Manufacturer::Wattbike),
            76 => Ok(Manufacturer::Moxy),
            77 => Ok(Manufacturer::Ciclosport),
            78 => Ok(Manufacturer::Powerbahn),
            79 => Ok(Manufacturer::AcornProjectsAps),
            80 => Ok(Manufacturer::Lifebeam),
            81 => Ok(Manufacturer::Bontrager),
            82 => Ok(Manufacturer::Wellgo),
            83 => Ok(Manufacturer::Scosche),
            84 => Ok(Manufacturer::Magura),
            85 => Ok(Manufacturer::Woodway),
            86 => Ok(Manufacturer::Elite),
            87 => Ok(Manufacturer::NielsenKellerman),
            88 => Ok(Manufacturer::DkCity),
            89 => Ok(Manufacturer::Tacx),
            90 => Ok(Manufacturer::DirectionTechnology),
            91 => Ok(Manufacturer::Magtonic),
            92 => Ok(Manufacturer::Onepartcarbon),
            93 => Ok(Manufacturer::InsideRideTechnologies),
            94 => Ok(Manufacturer::SoundOfMotion),
            95 => Ok(Manufacturer::Stryd),
            96 => Ok(Manufacturer::Icg),
            97 => Ok(Manufacturer::MiPulse),
            98 => Ok(Manufacturer::BsxAthletics),
            99 => Ok(Manufacturer::Look),
            100 => Ok(Manufacturer::CampagnoloSrl),
            101 => Ok(Manufacturer::BodyBikeSmart),
            102 => Ok(Manufacturer::Praxisworks),
            103 => Ok(Manufacturer::LimitsTechnology),
            104 => Ok(Manufacturer::TopactionTechnology),
            105 => Ok(Manufacturer::Cosinuss),
            106 => Ok(Manufacturer::Fitcare),
            107 => Ok(Manufacturer::Magene),
            108 => Ok(Manufacturer::GiantManufacturingCo),
            109 => Ok(Manufacturer::Tigrasport),
            110 => Ok(Manufacturer::Salutron),
            111 => Ok(Manufacturer::Technogym),
            112 => Ok(Manufacturer::BrytonSensors),
            113 => Ok(Manufacturer::LatitudeLimited),
            114 => Ok(Manufacturer::SoaringTechnology),
            115 => Ok(Manufacturer::Igpsport),
            116 => Ok(Manufacturer::Thinkrider),
            117 => Ok(Manufacturer::GopherSport),
            118 => Ok(Manufacturer::Waterrower),
            119 => Ok(Manufacturer::Orangetheory),
            120 => Ok(Manufacturer::Inpeak),
            121 => Ok(Manufacturer::Kinetic),
            122 => Ok(Manufacturer::JohnsonHealthTech),
            123 => Ok(Manufacturer::PolarElectro),
            124 => Ok(Manufacturer::Seesense),
            255 => Ok(Manufacturer::Development),
            257 => Ok(Manufacturer::Healthandlife),
            258 => Ok(Manufacturer::Lezyne),
            259 => Ok(Manufacturer::ScribeLabs),
            260 => Ok(Manufacturer::Zwift),
            261 => Ok(Manufacturer::Watteam),
            262 => Ok(Manufacturer::Recon),
            263 => Ok(Manufacturer::FaveroElectronics),
            264 => Ok(Manufacturer::Dynovelo),
            265 => Ok(Manufacturer::Strava),
            266 => Ok(Manufacturer::Precor),
            267 => Ok(Manufacturer::Bryton),
            268 => Ok(Manufacturer::Sram),
            269 => Ok(Manufacturer::Navman),
            270 => Ok(Manufacturer::Cobi),
            271 => Ok(Manufacturer::Spivi),
            272 => Ok(Manufacturer::MioMagellan),
            273 => Ok(Manufacturer::Evesports),
            274 => Ok(Manufacturer::SensitivusGauge),
            275 => Ok(Manufacturer::Podoon),
            276 => Ok(Manufacturer::LifeTimeFitness),
            277 => Ok(Manufacturer::FalcoEMotors),
            278 => Ok(Manufacturer::Minoura),
            279 => Ok(Manufacturer::Cycliq),
            280 => Ok(Manufacturer::Luxottica),
            281 => Ok(Manufacturer::TrainerRoad),
            282 => Ok(Manufacturer::TheSufferfest),
            283 => Ok(Manufacturer::Fullspeedahead),
            284 => Ok(Manufacturer::Virtualtraining),
            285 => Ok(Manufacturer::Feedbacksports),
            286 => Ok(Manufacturer::Omata),
            287 => Ok(Manufacturer::Vdo),
            288 => Ok(Manufacturer::Magneticdays),
            289 => Ok(Manufacturer::Hammerhead),
            290 => Ok(Manufacturer::KineticByKurt),
            291 => Ok(Manufacturer::Shapelog),
            292 => Ok(Manufacturer::Dabuziduo),
            293 => Ok(Manufacturer::Jetblack),
            5759 => Ok(Manufacturer::Actigraphcorp),
            _ => Ok(Manufacturer::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum GarminProduct {
    Hrm1 = 1,
    #[doc = "AXH01 HRM chipset"]
    Axh01 = 2,
    Axb01 = 3,
    Axb02 = 4,
    Hrm2Ss = 5,
    DsiAlf02 = 6,
    Hrm3Ss = 7,
    #[doc = "hrm_run model for HRM ANT+ messaging"]
    HrmRunSingleByteProductId = 8,
    #[doc = "BSM model for ANT+ messaging"]
    Bsm = 9,
    #[doc = "BCM model for ANT+ messaging"]
    Bcm = 10,
    #[doc = "AXS01 HRM Bike Chipset model for ANT+ messaging"]
    Axs01 = 11,
    #[doc = "hrm_tri model for HRM ANT+ messaging"]
    HrmTriSingleByteProductId = 12,
    #[doc = "fr225 model for HRM ANT+ messaging"]
    Fr225SingleByteProductId = 14,
    Fr301China = 473,
    Fr301Japan = 474,
    Fr301Korea = 475,
    Fr301Taiwan = 494,
    #[doc = "Forerunner 405"]
    Fr405 = 717,
    #[doc = "Forerunner 50"]
    Fr50 = 782,
    Fr405Japan = 987,
    #[doc = "Forerunner 60"]
    Fr60 = 988,
    DsiAlf01 = 1011,
    #[doc = "Forerunner 310"]
    Fr310Xt = 1018,
    Edge500 = 1036,
    #[doc = "Forerunner 110"]
    Fr110 = 1124,
    Edge800 = 1169,
    Edge500Taiwan = 1199,
    Edge500Japan = 1213,
    Chirp = 1253,
    Fr110Japan = 1274,
    Edge200 = 1325,
    Fr910Xt = 1328,
    Edge800Taiwan = 1333,
    Edge800Japan = 1334,
    Alf04 = 1341,
    Fr610 = 1345,
    Fr210Japan = 1360,
    VectorSs = 1380,
    VectorCp = 1381,
    Edge800China = 1386,
    Edge500China = 1387,
    Fr610Japan = 1410,
    Edge500Korea = 1422,
    Fr70 = 1436,
    Fr310Xt4T = 1446,
    Amx = 1461,
    Fr10 = 1482,
    Edge800Korea = 1497,
    Swim = 1499,
    Fr910XtChina = 1537,
    Fenix = 1551,
    Edge200Taiwan = 1555,
    Edge510 = 1561,
    Edge810 = 1567,
    Tempe = 1570,
    Fr910XtJapan = 1600,
    Fr620 = 1623,
    Fr220 = 1632,
    Fr910XtKorea = 1664,
    Fr10Japan = 1688,
    Edge810Japan = 1721,
    VirbElite = 1735,
    #[doc = "Also Edge Touring Plus"]
    EdgeTouring = 1736,
    Edge510Japan = 1742,
    HrmTri = 1743,
    HrmRun = 1752,
    Fr920Xt = 1765,
    Edge510Asia = 1821,
    Edge810China = 1822,
    Edge810Taiwan = 1823,
    Edge1000 = 1836,
    VivoFit = 1837,
    VirbRemote = 1853,
    VivoKi = 1885,
    Fr15 = 1903,
    VivoActive = 1907,
    Edge510Korea = 1918,
    Fr620Japan = 1928,
    Fr620China = 1929,
    Fr220Japan = 1930,
    Fr220China = 1931,
    ApproachS6 = 1936,
    VivoSmart = 1956,
    Fenix2 = 1967,
    Epix = 1988,
    Fenix3 = 2050,
    Edge1000Taiwan = 2052,
    Edge1000Japan = 2053,
    Fr15Japan = 2061,
    Edge520 = 2067,
    Edge1000China = 2070,
    Fr620Russia = 2072,
    Fr220Russia = 2073,
    VectorS = 2079,
    Edge1000Korea = 2100,
    Fr920XtTaiwan = 2130,
    Fr920XtChina = 2131,
    Fr920XtJapan = 2132,
    Virbx = 2134,
    VivoSmartApac = 2135,
    EtrexTouch = 2140,
    Edge25 = 2147,
    Fr25 = 2148,
    VivoFit2 = 2150,
    Fr225 = 2153,
    Fr630 = 2156,
    Fr230 = 2157,
    VivoActiveApac = 2160,
    Vector2 = 2161,
    Vector2S = 2162,
    Virbxe = 2172,
    Fr620Taiwan = 2173,
    Fr220Taiwan = 2174,
    Truswing = 2175,
    Fenix3China = 2188,
    Fenix3Twn = 2189,
    VariaHeadlight = 2192,
    VariaTaillightOld = 2193,
    EdgeExplore1000 = 2204,
    Fr225Asia = 2219,
    VariaRadarTaillight = 2225,
    VariaRadarDisplay = 2226,
    Edge20 = 2238,
    D2Bravo = 2262,
    ApproachS20 = 2266,
    VariaRemote = 2276,
    Hrm4Run = 2327,
    VivoActiveHr = 2337,
    VivoSmartGpsHr = 2347,
    VivoSmartHr = 2348,
    VivoMove = 2368,
    VariaVision = 2398,
    VivoFit3 = 2406,
    Fenix3Hr = 2413,
    VirbUltra30 = 2417,
    IndexSmartScale = 2429,
    Fr235 = 2431,
    Fenix3Chronos = 2432,
    Oregon7Xx = 2441,
    Rino7Xx = 2444,
    Nautix = 2496,
    Edge820 = 2530,
    EdgeExplore820 = 2531,
    Fenix5S = 2544,
    D2BravoTitanium = 2547,
    #[doc = "Varia UT 800 SW"]
    VariaUt800 = 2567,
    RunningDynamicsPod = 2593,
    Fenix5X = 2604,
    VivoFitJr = 2606,
    Fr935 = 2691,
    Fenix5 = 2697,
    #[doc = "SDM4 footpod"]
    Sdm4 = 10007,
    EdgeRemote = 10014,
    TrainingCenter = 20119,
    ConnectiqSimulator = 65531,
    AndroidAntplusPlugin = 65532,
    #[doc = "Garmin Connect website"]
    Connect = 65534,
    Unknown,
}
impl GarminProduct {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(GarminProduct::Hrm1),
            2 => Ok(GarminProduct::Axh01),
            3 => Ok(GarminProduct::Axb01),
            4 => Ok(GarminProduct::Axb02),
            5 => Ok(GarminProduct::Hrm2Ss),
            6 => Ok(GarminProduct::DsiAlf02),
            7 => Ok(GarminProduct::Hrm3Ss),
            8 => Ok(GarminProduct::HrmRunSingleByteProductId),
            9 => Ok(GarminProduct::Bsm),
            10 => Ok(GarminProduct::Bcm),
            11 => Ok(GarminProduct::Axs01),
            12 => Ok(GarminProduct::HrmTriSingleByteProductId),
            14 => Ok(GarminProduct::Fr225SingleByteProductId),
            473 => Ok(GarminProduct::Fr301China),
            474 => Ok(GarminProduct::Fr301Japan),
            475 => Ok(GarminProduct::Fr301Korea),
            494 => Ok(GarminProduct::Fr301Taiwan),
            717 => Ok(GarminProduct::Fr405),
            782 => Ok(GarminProduct::Fr50),
            987 => Ok(GarminProduct::Fr405Japan),
            988 => Ok(GarminProduct::Fr60),
            1011 => Ok(GarminProduct::DsiAlf01),
            1018 => Ok(GarminProduct::Fr310Xt),
            1036 => Ok(GarminProduct::Edge500),
            1124 => Ok(GarminProduct::Fr110),
            1169 => Ok(GarminProduct::Edge800),
            1199 => Ok(GarminProduct::Edge500Taiwan),
            1213 => Ok(GarminProduct::Edge500Japan),
            1253 => Ok(GarminProduct::Chirp),
            1274 => Ok(GarminProduct::Fr110Japan),
            1325 => Ok(GarminProduct::Edge200),
            1328 => Ok(GarminProduct::Fr910Xt),
            1333 => Ok(GarminProduct::Edge800Taiwan),
            1334 => Ok(GarminProduct::Edge800Japan),
            1341 => Ok(GarminProduct::Alf04),
            1345 => Ok(GarminProduct::Fr610),
            1360 => Ok(GarminProduct::Fr210Japan),
            1380 => Ok(GarminProduct::VectorSs),
            1381 => Ok(GarminProduct::VectorCp),
            1386 => Ok(GarminProduct::Edge800China),
            1387 => Ok(GarminProduct::Edge500China),
            1410 => Ok(GarminProduct::Fr610Japan),
            1422 => Ok(GarminProduct::Edge500Korea),
            1436 => Ok(GarminProduct::Fr70),
            1446 => Ok(GarminProduct::Fr310Xt4T),
            1461 => Ok(GarminProduct::Amx),
            1482 => Ok(GarminProduct::Fr10),
            1497 => Ok(GarminProduct::Edge800Korea),
            1499 => Ok(GarminProduct::Swim),
            1537 => Ok(GarminProduct::Fr910XtChina),
            1551 => Ok(GarminProduct::Fenix),
            1555 => Ok(GarminProduct::Edge200Taiwan),
            1561 => Ok(GarminProduct::Edge510),
            1567 => Ok(GarminProduct::Edge810),
            1570 => Ok(GarminProduct::Tempe),
            1600 => Ok(GarminProduct::Fr910XtJapan),
            1623 => Ok(GarminProduct::Fr620),
            1632 => Ok(GarminProduct::Fr220),
            1664 => Ok(GarminProduct::Fr910XtKorea),
            1688 => Ok(GarminProduct::Fr10Japan),
            1721 => Ok(GarminProduct::Edge810Japan),
            1735 => Ok(GarminProduct::VirbElite),
            1736 => Ok(GarminProduct::EdgeTouring),
            1742 => Ok(GarminProduct::Edge510Japan),
            1743 => Ok(GarminProduct::HrmTri),
            1752 => Ok(GarminProduct::HrmRun),
            1765 => Ok(GarminProduct::Fr920Xt),
            1821 => Ok(GarminProduct::Edge510Asia),
            1822 => Ok(GarminProduct::Edge810China),
            1823 => Ok(GarminProduct::Edge810Taiwan),
            1836 => Ok(GarminProduct::Edge1000),
            1837 => Ok(GarminProduct::VivoFit),
            1853 => Ok(GarminProduct::VirbRemote),
            1885 => Ok(GarminProduct::VivoKi),
            1903 => Ok(GarminProduct::Fr15),
            1907 => Ok(GarminProduct::VivoActive),
            1918 => Ok(GarminProduct::Edge510Korea),
            1928 => Ok(GarminProduct::Fr620Japan),
            1929 => Ok(GarminProduct::Fr620China),
            1930 => Ok(GarminProduct::Fr220Japan),
            1931 => Ok(GarminProduct::Fr220China),
            1936 => Ok(GarminProduct::ApproachS6),
            1956 => Ok(GarminProduct::VivoSmart),
            1967 => Ok(GarminProduct::Fenix2),
            1988 => Ok(GarminProduct::Epix),
            2050 => Ok(GarminProduct::Fenix3),
            2052 => Ok(GarminProduct::Edge1000Taiwan),
            2053 => Ok(GarminProduct::Edge1000Japan),
            2061 => Ok(GarminProduct::Fr15Japan),
            2067 => Ok(GarminProduct::Edge520),
            2070 => Ok(GarminProduct::Edge1000China),
            2072 => Ok(GarminProduct::Fr620Russia),
            2073 => Ok(GarminProduct::Fr220Russia),
            2079 => Ok(GarminProduct::VectorS),
            2100 => Ok(GarminProduct::Edge1000Korea),
            2130 => Ok(GarminProduct::Fr920XtTaiwan),
            2131 => Ok(GarminProduct::Fr920XtChina),
            2132 => Ok(GarminProduct::Fr920XtJapan),
            2134 => Ok(GarminProduct::Virbx),
            2135 => Ok(GarminProduct::VivoSmartApac),
            2140 => Ok(GarminProduct::EtrexTouch),
            2147 => Ok(GarminProduct::Edge25),
            2148 => Ok(GarminProduct::Fr25),
            2150 => Ok(GarminProduct::VivoFit2),
            2153 => Ok(GarminProduct::Fr225),
            2156 => Ok(GarminProduct::Fr630),
            2157 => Ok(GarminProduct::Fr230),
            2160 => Ok(GarminProduct::VivoActiveApac),
            2161 => Ok(GarminProduct::Vector2),
            2162 => Ok(GarminProduct::Vector2S),
            2172 => Ok(GarminProduct::Virbxe),
            2173 => Ok(GarminProduct::Fr620Taiwan),
            2174 => Ok(GarminProduct::Fr220Taiwan),
            2175 => Ok(GarminProduct::Truswing),
            2188 => Ok(GarminProduct::Fenix3China),
            2189 => Ok(GarminProduct::Fenix3Twn),
            2192 => Ok(GarminProduct::VariaHeadlight),
            2193 => Ok(GarminProduct::VariaTaillightOld),
            2204 => Ok(GarminProduct::EdgeExplore1000),
            2219 => Ok(GarminProduct::Fr225Asia),
            2225 => Ok(GarminProduct::VariaRadarTaillight),
            2226 => Ok(GarminProduct::VariaRadarDisplay),
            2238 => Ok(GarminProduct::Edge20),
            2262 => Ok(GarminProduct::D2Bravo),
            2266 => Ok(GarminProduct::ApproachS20),
            2276 => Ok(GarminProduct::VariaRemote),
            2327 => Ok(GarminProduct::Hrm4Run),
            2337 => Ok(GarminProduct::VivoActiveHr),
            2347 => Ok(GarminProduct::VivoSmartGpsHr),
            2348 => Ok(GarminProduct::VivoSmartHr),
            2368 => Ok(GarminProduct::VivoMove),
            2398 => Ok(GarminProduct::VariaVision),
            2406 => Ok(GarminProduct::VivoFit3),
            2413 => Ok(GarminProduct::Fenix3Hr),
            2417 => Ok(GarminProduct::VirbUltra30),
            2429 => Ok(GarminProduct::IndexSmartScale),
            2431 => Ok(GarminProduct::Fr235),
            2432 => Ok(GarminProduct::Fenix3Chronos),
            2441 => Ok(GarminProduct::Oregon7Xx),
            2444 => Ok(GarminProduct::Rino7Xx),
            2496 => Ok(GarminProduct::Nautix),
            2530 => Ok(GarminProduct::Edge820),
            2531 => Ok(GarminProduct::EdgeExplore820),
            2544 => Ok(GarminProduct::Fenix5S),
            2547 => Ok(GarminProduct::D2BravoTitanium),
            2567 => Ok(GarminProduct::VariaUt800),
            2593 => Ok(GarminProduct::RunningDynamicsPod),
            2604 => Ok(GarminProduct::Fenix5X),
            2606 => Ok(GarminProduct::VivoFitJr),
            2691 => Ok(GarminProduct::Fr935),
            2697 => Ok(GarminProduct::Fenix5),
            10007 => Ok(GarminProduct::Sdm4),
            10014 => Ok(GarminProduct::EdgeRemote),
            20119 => Ok(GarminProduct::TrainingCenter),
            65531 => Ok(GarminProduct::ConnectiqSimulator),
            65532 => Ok(GarminProduct::AndroidAntplusPlugin),
            65534 => Ok(GarminProduct::Connect),
            _ => Ok(GarminProduct::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum AntplusDeviceType {
    Antfs = 1,
    BikePower = 11,
    EnvironmentSensorLegacy = 12,
    MultiSportSpeedDistance = 15,
    Control = 16,
    FitnessEquipment = 17,
    BloodPressure = 18,
    GeocacheNode = 19,
    LightElectricVehicle = 20,
    EnvSensor = 25,
    Racquet = 26,
    ControlHub = 27,
    MuscleOxygen = 31,
    BikeLightMain = 35,
    BikeLightShared = 36,
    Exd = 38,
    BikeRadar = 40,
    WeightScale = 119,
    HeartRate = 120,
    BikeSpeedCadence = 121,
    BikeCadence = 122,
    BikeSpeed = 123,
    StrideSpeedDistance = 124,
    Unknown,
}
impl AntplusDeviceType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(AntplusDeviceType::Antfs),
            11 => Ok(AntplusDeviceType::BikePower),
            12 => Ok(AntplusDeviceType::EnvironmentSensorLegacy),
            15 => Ok(AntplusDeviceType::MultiSportSpeedDistance),
            16 => Ok(AntplusDeviceType::Control),
            17 => Ok(AntplusDeviceType::FitnessEquipment),
            18 => Ok(AntplusDeviceType::BloodPressure),
            19 => Ok(AntplusDeviceType::GeocacheNode),
            20 => Ok(AntplusDeviceType::LightElectricVehicle),
            25 => Ok(AntplusDeviceType::EnvSensor),
            26 => Ok(AntplusDeviceType::Racquet),
            27 => Ok(AntplusDeviceType::ControlHub),
            31 => Ok(AntplusDeviceType::MuscleOxygen),
            35 => Ok(AntplusDeviceType::BikeLightMain),
            36 => Ok(AntplusDeviceType::BikeLightShared),
            38 => Ok(AntplusDeviceType::Exd),
            40 => Ok(AntplusDeviceType::BikeRadar),
            119 => Ok(AntplusDeviceType::WeightScale),
            120 => Ok(AntplusDeviceType::HeartRate),
            121 => Ok(AntplusDeviceType::BikeSpeedCadence),
            122 => Ok(AntplusDeviceType::BikeCadence),
            123 => Ok(AntplusDeviceType::BikeSpeed),
            124 => Ok(AntplusDeviceType::StrideSpeedDistance),
            _ => Ok(AntplusDeviceType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum AntNetwork {
    Public = 0,
    Antplus = 1,
    Antfs = 2,
    Private = 3,
    Unknown,
}
impl AntNetwork {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(AntNetwork::Public),
            1 => Ok(AntNetwork::Antplus),
            2 => Ok(AntNetwork::Antfs),
            3 => Ok(AntNetwork::Private),
            _ => Ok(AntNetwork::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum WorkoutCapabilities {
    Interval = 1,
    Custom = 2,
    FitnessEquipment = 4,
    Firstbeat = 8,
    NewLeaf = 16,
    #[doc = "For backwards compatibility.  Watch should add missing id fields \
             then clear flag."]
    Tcx = 32,
    #[doc = "Speed source required for workout step."]
    Speed = 128,
    #[doc = "Heart rate source required for workout step."]
    HeartRate = 256,
    #[doc = "Distance source required for workout step."]
    Distance = 512,
    #[doc = "Cadence source required for workout step."]
    Cadence = 1024,
    #[doc = "Power source required for workout step."]
    Power = 2048,
    #[doc = "Grade source required for workout step."]
    Grade = 4096,
    #[doc = "Resistance source required for workout step."]
    Resistance = 8192,
    Protected = 16384,
    Unknown,
}
impl WorkoutCapabilities {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint32z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(WorkoutCapabilities::Interval),
            2 => Ok(WorkoutCapabilities::Custom),
            4 => Ok(WorkoutCapabilities::FitnessEquipment),
            8 => Ok(WorkoutCapabilities::Firstbeat),
            16 => Ok(WorkoutCapabilities::NewLeaf),
            32 => Ok(WorkoutCapabilities::Tcx),
            128 => Ok(WorkoutCapabilities::Speed),
            256 => Ok(WorkoutCapabilities::HeartRate),
            512 => Ok(WorkoutCapabilities::Distance),
            1024 => Ok(WorkoutCapabilities::Cadence),
            2048 => Ok(WorkoutCapabilities::Power),
            4096 => Ok(WorkoutCapabilities::Grade),
            8192 => Ok(WorkoutCapabilities::Resistance),
            16384 => Ok(WorkoutCapabilities::Protected),
            _ => Ok(WorkoutCapabilities::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum BatteryStatus {
    New = 1,
    Good = 2,
    Ok = 3,
    Low = 4,
    Critical = 5,
    Charging = 6,
    Unknown,
}
impl BatteryStatus {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(BatteryStatus::New),
            2 => Ok(BatteryStatus::Good),
            3 => Ok(BatteryStatus::Ok),
            4 => Ok(BatteryStatus::Low),
            5 => Ok(BatteryStatus::Critical),
            6 => Ok(BatteryStatus::Charging),
            7 => Ok(BatteryStatus::Unknown),
            _ => Ok(BatteryStatus::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum HrType {
    Normal = 0,
    Irregular = 1,
    Unknown,
}
impl HrType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(HrType::Normal),
            1 => Ok(HrType::Irregular),
            _ => Ok(HrType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum CourseCapabilities {
    Processed = 1,
    Valid = 2,
    Time = 4,
    Distance = 8,
    Position = 16,
    HeartRate = 32,
    Power = 64,
    Cadence = 128,
    Training = 256,
    Navigation = 512,
    Bikeway = 1024,
    Unknown,
}
impl CourseCapabilities {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint32z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(CourseCapabilities::Processed),
            2 => Ok(CourseCapabilities::Valid),
            4 => Ok(CourseCapabilities::Time),
            8 => Ok(CourseCapabilities::Distance),
            16 => Ok(CourseCapabilities::Position),
            32 => Ok(CourseCapabilities::HeartRate),
            64 => Ok(CourseCapabilities::Power),
            128 => Ok(CourseCapabilities::Cadence),
            256 => Ok(CourseCapabilities::Training),
            512 => Ok(CourseCapabilities::Navigation),
            1024 => Ok(CourseCapabilities::Bikeway),
            _ => Ok(CourseCapabilities::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Weight {
    Calculating = 65534,
    Unknown,
}
impl Weight {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            65534 => Ok(Weight::Calculating),
            _ => Ok(Weight::Unknown),
        }
    }
}
#[doc = "0 - 100 indicates% of max hr; >100 indicates bpm (255 max) plus 100"]
#[derive(Debug)]
pub enum WorkoutHr {
    BpmOffset = 100,
    Unknown,
}
impl WorkoutHr {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint32::decode::<T>(buffer)?;
        match base_value.0 {
            100 => Ok(WorkoutHr::BpmOffset),
            _ => Ok(WorkoutHr::Unknown),
        }
    }
}
#[doc = "0 - 1000 indicates % of functional threshold power; >1000 indicates \
         watts plus 1000."]
#[derive(Debug)]
pub enum WorkoutPower {
    WattsOffset = 1000,
    Unknown,
}
impl WorkoutPower {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint32::decode::<T>(buffer)?;
        match base_value.0 {
            1000 => Ok(WorkoutPower::WattsOffset),
            _ => Ok(WorkoutPower::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum BpStatus {
    NoError = 0,
    ErrorIncompleteData = 1,
    ErrorNoMeasurement = 2,
    ErrorDataOutOfRange = 3,
    ErrorIrregularHeartRate = 4,
    Unknown,
}
impl BpStatus {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(BpStatus::NoError),
            1 => Ok(BpStatus::ErrorIncompleteData),
            2 => Ok(BpStatus::ErrorNoMeasurement),
            3 => Ok(BpStatus::ErrorDataOutOfRange),
            4 => Ok(BpStatus::ErrorIrregularHeartRate),
            _ => Ok(BpStatus::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum UserLocalId {
    LocalMin = 0,
    LocalMax = 15,
    StationaryMin = 16,
    StationaryMax = 255,
    PortableMin = 256,
    PortableMax = 65534,
    Unknown,
}
impl UserLocalId {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(UserLocalId::LocalMin),
            15 => Ok(UserLocalId::LocalMax),
            16 => Ok(UserLocalId::StationaryMin),
            255 => Ok(UserLocalId::StationaryMax),
            256 => Ok(UserLocalId::PortableMin),
            65534 => Ok(UserLocalId::PortableMax),
            _ => Ok(UserLocalId::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SwimStroke {
    Freestyle = 0,
    Backstroke = 1,
    Breaststroke = 2,
    Butterfly = 3,
    Drill = 4,
    Mixed = 5,
    #[doc = "IM is a mixed interval containing the same number of lengths for \
             each of: Butterfly, Backstroke, Breaststroke, Freestyle, swam in \
             that order."]
    Im = 6,
    Unknown,
}
impl SwimStroke {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SwimStroke::Freestyle),
            1 => Ok(SwimStroke::Backstroke),
            2 => Ok(SwimStroke::Breaststroke),
            3 => Ok(SwimStroke::Butterfly),
            4 => Ok(SwimStroke::Drill),
            5 => Ok(SwimStroke::Mixed),
            6 => Ok(SwimStroke::Im),
            _ => Ok(SwimStroke::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ActivityType {
    Generic = 0,
    Running = 1,
    Cycling = 2,
    #[doc = "Mulitsport transition"]
    Transition = 3,
    FitnessEquipment = 4,
    Swimming = 5,
    Walking = 6,
    Sedentary = 8,
    #[doc = "All is for goals only to include all sports."]
    All = 254,
    Unknown,
}
impl ActivityType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ActivityType::Generic),
            1 => Ok(ActivityType::Running),
            2 => Ok(ActivityType::Cycling),
            3 => Ok(ActivityType::Transition),
            4 => Ok(ActivityType::FitnessEquipment),
            5 => Ok(ActivityType::Swimming),
            6 => Ok(ActivityType::Walking),
            8 => Ok(ActivityType::Sedentary),
            254 => Ok(ActivityType::All),
            _ => Ok(ActivityType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ActivitySubtype {
    Generic = 0,
    #[doc = "Run"]
    Treadmill = 1,
    #[doc = "Run"]
    Street = 2,
    #[doc = "Run"]
    Trail = 3,
    #[doc = "Run"]
    Track = 4,
    #[doc = "Cycling"]
    Spin = 5,
    #[doc = "Cycling"]
    IndoorCycling = 6,
    #[doc = "Cycling"]
    Road = 7,
    #[doc = "Cycling"]
    Mountain = 8,
    #[doc = "Cycling"]
    Downhill = 9,
    #[doc = "Cycling"]
    Recumbent = 10,
    #[doc = "Cycling"]
    Cyclocross = 11,
    #[doc = "Cycling"]
    HandCycling = 12,
    #[doc = "Cycling"]
    TrackCycling = 13,
    #[doc = "Fitness Equipment"]
    IndoorRowing = 14,
    #[doc = "Fitness Equipment"]
    Elliptical = 15,
    #[doc = "Fitness Equipment"]
    StairClimbing = 16,
    #[doc = "Swimming"]
    LapSwimming = 17,
    #[doc = "Swimming"]
    OpenWater = 18,
    All = 254,
    Unknown,
}
impl ActivitySubtype {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ActivitySubtype::Generic),
            1 => Ok(ActivitySubtype::Treadmill),
            2 => Ok(ActivitySubtype::Street),
            3 => Ok(ActivitySubtype::Trail),
            4 => Ok(ActivitySubtype::Track),
            5 => Ok(ActivitySubtype::Spin),
            6 => Ok(ActivitySubtype::IndoorCycling),
            7 => Ok(ActivitySubtype::Road),
            8 => Ok(ActivitySubtype::Mountain),
            9 => Ok(ActivitySubtype::Downhill),
            10 => Ok(ActivitySubtype::Recumbent),
            11 => Ok(ActivitySubtype::Cyclocross),
            12 => Ok(ActivitySubtype::HandCycling),
            13 => Ok(ActivitySubtype::TrackCycling),
            14 => Ok(ActivitySubtype::IndoorRowing),
            15 => Ok(ActivitySubtype::Elliptical),
            16 => Ok(ActivitySubtype::StairClimbing),
            17 => Ok(ActivitySubtype::LapSwimming),
            18 => Ok(ActivitySubtype::OpenWater),
            254 => Ok(ActivitySubtype::All),
            _ => Ok(ActivitySubtype::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ActivityLevel {
    Low = 0,
    Medium = 1,
    High = 2,
    Unknown,
}
impl ActivityLevel {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ActivityLevel::Low),
            1 => Ok(ActivityLevel::Medium),
            2 => Ok(ActivityLevel::High),
            _ => Ok(ActivityLevel::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum Side {
    Right = 0,
    Left = 1,
    Unknown,
}
impl Side {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(Side::Right),
            1 => Ok(Side::Left),
            _ => Ok(Side::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LeftRightBalance {
    #[doc = "% contribution"]
    Mask = 127,
    #[doc = "data corresponds to right if set, otherwise unknown"]
    Right = 128,
    Unknown,
}
impl LeftRightBalance {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8::decode::<T>(buffer)?;
        match base_value.0 {
            127 => Ok(LeftRightBalance::Mask),
            128 => Ok(LeftRightBalance::Right),
            _ => Ok(LeftRightBalance::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LeftRightBalance100 {
    #[doc = "% contribution scaled by 100"]
    Mask = 16383,
    #[doc = "data corresponds to right if set, otherwise unknown"]
    Right = 32768,
    Unknown,
}
impl LeftRightBalance100 {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            16383 => Ok(LeftRightBalance100::Mask),
            32768 => Ok(LeftRightBalance100::Right),
            _ => Ok(LeftRightBalance100::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LengthType {
    #[doc = "Rest period. Length with no strokes"]
    Idle = 0,
    #[doc = "Length with strokes."]
    Active = 1,
    Unknown,
}
impl LengthType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(LengthType::Idle),
            1 => Ok(LengthType::Active),
            _ => Ok(LengthType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Unknown,
}
impl DayOfWeek {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DayOfWeek::Sunday),
            1 => Ok(DayOfWeek::Monday),
            2 => Ok(DayOfWeek::Tuesday),
            3 => Ok(DayOfWeek::Wednesday),
            4 => Ok(DayOfWeek::Thursday),
            5 => Ok(DayOfWeek::Friday),
            6 => Ok(DayOfWeek::Saturday),
            _ => Ok(DayOfWeek::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ConnectivityCapabilities {
    Bluetooth = 1,
    BluetoothLe = 2,
    Ant = 4,
    ActivityUpload = 8,
    CourseDownload = 16,
    WorkoutDownload = 32,
    LiveTrack = 64,
    WeatherConditions = 128,
    WeatherAlerts = 256,
    GpsEphemerisDownload = 512,
    ExplicitArchive = 1024,
    SetupIncomplete = 2048,
    ContinueSyncAfterSoftwareUpdate = 4096,
    ConnectIqAppDownload = 8192,
    GolfCourseDownload = 16384,
    #[doc = "Indicates device is in control of initiating all syncs"]
    DeviceInitiatesSync = 32768,
    ConnectIqWatchAppDownload = 65536,
    ConnectIqWidgetDownload = 131072,
    ConnectIqWatchFaceDownload = 262144,
    ConnectIqDataFieldDownload = 524288,
    #[doc = "Device supports delete and reorder of apps via GCM"]
    ConnectIqAppManagment = 1048576,
    SwingSensor = 2097152,
    SwingSensorRemote = 4194304,
    #[doc = "Device supports incident detection"]
    IncidentDetection = 8388608,
    AudioPrompts = 16777216,
    #[doc = "Device supports reporting wifi verification via GCM"]
    WifiVerification = 33554432,
    #[doc = "Device supports True Up"]
    TrueUp = 67108864,
    #[doc = "Device supports Find My Watch"]
    FindMyWatch = 134217728,
    RemoteManualSync = 268435456,
    #[doc = "Device supports LiveTrack auto start"]
    LiveTrackAutoStart = 536870912,
    #[doc = "Device supports LiveTrack Messaging"]
    LiveTrackMessaging = 1073741824,
    #[doc = "Device supports instant input feature"]
    InstantInput = 2147483648,
    Unknown,
}
impl ConnectivityCapabilities {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint32z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(ConnectivityCapabilities::Bluetooth),
            2 => Ok(ConnectivityCapabilities::BluetoothLe),
            4 => Ok(ConnectivityCapabilities::Ant),
            8 => Ok(ConnectivityCapabilities::ActivityUpload),
            16 => Ok(ConnectivityCapabilities::CourseDownload),
            32 => Ok(ConnectivityCapabilities::WorkoutDownload),
            64 => Ok(ConnectivityCapabilities::LiveTrack),
            128 => Ok(ConnectivityCapabilities::WeatherConditions),
            256 => Ok(ConnectivityCapabilities::WeatherAlerts),
            512 => Ok(ConnectivityCapabilities::GpsEphemerisDownload),
            1024 => Ok(ConnectivityCapabilities::ExplicitArchive),
            2048 => Ok(ConnectivityCapabilities::SetupIncomplete),
            4096 => {
                Ok(ConnectivityCapabilities::ContinueSyncAfterSoftwareUpdate)
            },
            8192 => Ok(ConnectivityCapabilities::ConnectIqAppDownload),
            16384 => Ok(ConnectivityCapabilities::GolfCourseDownload),
            32768 => Ok(ConnectivityCapabilities::DeviceInitiatesSync),
            65536 => Ok(ConnectivityCapabilities::ConnectIqWatchAppDownload),
            131072 => Ok(ConnectivityCapabilities::ConnectIqWidgetDownload),
            262144 => Ok(ConnectivityCapabilities::ConnectIqWatchFaceDownload),
            524288 => Ok(ConnectivityCapabilities::ConnectIqDataFieldDownload),
            1048576 => Ok(ConnectivityCapabilities::ConnectIqAppManagment),
            2097152 => Ok(ConnectivityCapabilities::SwingSensor),
            4194304 => Ok(ConnectivityCapabilities::SwingSensorRemote),
            8388608 => Ok(ConnectivityCapabilities::IncidentDetection),
            16777216 => Ok(ConnectivityCapabilities::AudioPrompts),
            33554432 => Ok(ConnectivityCapabilities::WifiVerification),
            67108864 => Ok(ConnectivityCapabilities::TrueUp),
            134217728 => Ok(ConnectivityCapabilities::FindMyWatch),
            268435456 => Ok(ConnectivityCapabilities::RemoteManualSync),
            536870912 => Ok(ConnectivityCapabilities::LiveTrackAutoStart),
            1073741824 => Ok(ConnectivityCapabilities::LiveTrackMessaging),
            2147483648 => Ok(ConnectivityCapabilities::InstantInput),
            _ => Ok(ConnectivityCapabilities::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum WeatherReport {
    Current = 0,
    HourlyForecast = 1,
    DailyForecast = 2,
    Unknown,
}
impl WeatherReport {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(WeatherReport::Current),
            1 => Ok(WeatherReport::HourlyForecast),
            2 => Ok(WeatherReport::DailyForecast),
            _ => Ok(WeatherReport::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum WeatherStatus {
    Clear = 0,
    PartlyCloudy = 1,
    MostlyCloudy = 2,
    Rain = 3,
    Snow = 4,
    Windy = 5,
    Thunderstorms = 6,
    WintryMix = 7,
    Fog = 8,
    Hazy = 11,
    Hail = 12,
    ScatteredShowers = 13,
    ScatteredThunderstorms = 14,
    UnknownPrecipitation = 15,
    LightRain = 16,
    HeavyRain = 17,
    LightSnow = 18,
    HeavySnow = 19,
    LightRainSnow = 20,
    HeavyRainSnow = 21,
    Cloudy = 22,
    Unknown,
}
impl WeatherStatus {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(WeatherStatus::Clear),
            1 => Ok(WeatherStatus::PartlyCloudy),
            2 => Ok(WeatherStatus::MostlyCloudy),
            3 => Ok(WeatherStatus::Rain),
            4 => Ok(WeatherStatus::Snow),
            5 => Ok(WeatherStatus::Windy),
            6 => Ok(WeatherStatus::Thunderstorms),
            7 => Ok(WeatherStatus::WintryMix),
            8 => Ok(WeatherStatus::Fog),
            11 => Ok(WeatherStatus::Hazy),
            12 => Ok(WeatherStatus::Hail),
            13 => Ok(WeatherStatus::ScatteredShowers),
            14 => Ok(WeatherStatus::ScatteredThunderstorms),
            15 => Ok(WeatherStatus::UnknownPrecipitation),
            16 => Ok(WeatherStatus::LightRain),
            17 => Ok(WeatherStatus::HeavyRain),
            18 => Ok(WeatherStatus::LightSnow),
            19 => Ok(WeatherStatus::HeavySnow),
            20 => Ok(WeatherStatus::LightRainSnow),
            21 => Ok(WeatherStatus::HeavyRainSnow),
            22 => Ok(WeatherStatus::Cloudy),
            _ => Ok(WeatherStatus::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum WeatherSeverity {
    Warning = 1,
    Watch = 2,
    Advisory = 3,
    Statement = 4,
    Unknown,
}
impl WeatherSeverity {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(WeatherSeverity::Unknown),
            1 => Ok(WeatherSeverity::Warning),
            2 => Ok(WeatherSeverity::Watch),
            3 => Ok(WeatherSeverity::Advisory),
            4 => Ok(WeatherSeverity::Statement),
            _ => Ok(WeatherSeverity::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum WeatherSevereType {
    Unspecified = 0,
    Tornado = 1,
    Tsunami = 2,
    Hurricane = 3,
    ExtremeWind = 4,
    Typhoon = 5,
    InlandHurricane = 6,
    HurricaneForceWind = 7,
    Waterspout = 8,
    SevereThunderstorm = 9,
    WreckhouseWinds = 10,
    LesSuetesWind = 11,
    Avalanche = 12,
    FlashFlood = 13,
    TropicalStorm = 14,
    InlandTropicalStorm = 15,
    Blizzard = 16,
    IceStorm = 17,
    FreezingRain = 18,
    DebrisFlow = 19,
    FlashFreeze = 20,
    DustStorm = 21,
    HighWind = 22,
    WinterStorm = 23,
    HeavyFreezingSpray = 24,
    ExtremeCold = 25,
    WindChill = 26,
    ColdWave = 27,
    HeavySnowAlert = 28,
    LakeEffectBlowingSnow = 29,
    SnowSquall = 30,
    LakeEffectSnow = 31,
    WinterWeather = 32,
    Sleet = 33,
    Snowfall = 34,
    SnowAndBlowingSnow = 35,
    BlowingSnow = 36,
    SnowAlert = 37,
    ArcticOutflow = 38,
    FreezingDrizzle = 39,
    Storm = 40,
    StormSurge = 41,
    Rainfall = 42,
    ArealFlood = 43,
    CoastalFlood = 44,
    LakeshoreFlood = 45,
    ExcessiveHeat = 46,
    Heat = 47,
    Weather = 48,
    HighHeatAndHumidity = 49,
    HumidexAndHealth = 50,
    Humidex = 51,
    Gale = 52,
    FreezingSpray = 53,
    SpecialMarine = 54,
    Squall = 55,
    StrongWind = 56,
    LakeWind = 57,
    MarineWeather = 58,
    Wind = 59,
    SmallCraftHazardousSeas = 60,
    HazardousSeas = 61,
    SmallCraft = 62,
    SmallCraftWinds = 63,
    SmallCraftRoughBar = 64,
    HighWaterLevel = 65,
    Ashfall = 66,
    FreezingFog = 67,
    DenseFog = 68,
    DenseSmoke = 69,
    BlowingDust = 70,
    HardFreeze = 71,
    Freeze = 72,
    Frost = 73,
    FireWeather = 74,
    Flood = 75,
    RipTide = 76,
    HighSurf = 77,
    Smog = 78,
    AirQuality = 79,
    BriskWind = 80,
    AirStagnation = 81,
    LowWater = 82,
    Hydrological = 83,
    SpecialWeather = 84,
    Unknown,
}
impl WeatherSevereType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(WeatherSevereType::Unspecified),
            1 => Ok(WeatherSevereType::Tornado),
            2 => Ok(WeatherSevereType::Tsunami),
            3 => Ok(WeatherSevereType::Hurricane),
            4 => Ok(WeatherSevereType::ExtremeWind),
            5 => Ok(WeatherSevereType::Typhoon),
            6 => Ok(WeatherSevereType::InlandHurricane),
            7 => Ok(WeatherSevereType::HurricaneForceWind),
            8 => Ok(WeatherSevereType::Waterspout),
            9 => Ok(WeatherSevereType::SevereThunderstorm),
            10 => Ok(WeatherSevereType::WreckhouseWinds),
            11 => Ok(WeatherSevereType::LesSuetesWind),
            12 => Ok(WeatherSevereType::Avalanche),
            13 => Ok(WeatherSevereType::FlashFlood),
            14 => Ok(WeatherSevereType::TropicalStorm),
            15 => Ok(WeatherSevereType::InlandTropicalStorm),
            16 => Ok(WeatherSevereType::Blizzard),
            17 => Ok(WeatherSevereType::IceStorm),
            18 => Ok(WeatherSevereType::FreezingRain),
            19 => Ok(WeatherSevereType::DebrisFlow),
            20 => Ok(WeatherSevereType::FlashFreeze),
            21 => Ok(WeatherSevereType::DustStorm),
            22 => Ok(WeatherSevereType::HighWind),
            23 => Ok(WeatherSevereType::WinterStorm),
            24 => Ok(WeatherSevereType::HeavyFreezingSpray),
            25 => Ok(WeatherSevereType::ExtremeCold),
            26 => Ok(WeatherSevereType::WindChill),
            27 => Ok(WeatherSevereType::ColdWave),
            28 => Ok(WeatherSevereType::HeavySnowAlert),
            29 => Ok(WeatherSevereType::LakeEffectBlowingSnow),
            30 => Ok(WeatherSevereType::SnowSquall),
            31 => Ok(WeatherSevereType::LakeEffectSnow),
            32 => Ok(WeatherSevereType::WinterWeather),
            33 => Ok(WeatherSevereType::Sleet),
            34 => Ok(WeatherSevereType::Snowfall),
            35 => Ok(WeatherSevereType::SnowAndBlowingSnow),
            36 => Ok(WeatherSevereType::BlowingSnow),
            37 => Ok(WeatherSevereType::SnowAlert),
            38 => Ok(WeatherSevereType::ArcticOutflow),
            39 => Ok(WeatherSevereType::FreezingDrizzle),
            40 => Ok(WeatherSevereType::Storm),
            41 => Ok(WeatherSevereType::StormSurge),
            42 => Ok(WeatherSevereType::Rainfall),
            43 => Ok(WeatherSevereType::ArealFlood),
            44 => Ok(WeatherSevereType::CoastalFlood),
            45 => Ok(WeatherSevereType::LakeshoreFlood),
            46 => Ok(WeatherSevereType::ExcessiveHeat),
            47 => Ok(WeatherSevereType::Heat),
            48 => Ok(WeatherSevereType::Weather),
            49 => Ok(WeatherSevereType::HighHeatAndHumidity),
            50 => Ok(WeatherSevereType::HumidexAndHealth),
            51 => Ok(WeatherSevereType::Humidex),
            52 => Ok(WeatherSevereType::Gale),
            53 => Ok(WeatherSevereType::FreezingSpray),
            54 => Ok(WeatherSevereType::SpecialMarine),
            55 => Ok(WeatherSevereType::Squall),
            56 => Ok(WeatherSevereType::StrongWind),
            57 => Ok(WeatherSevereType::LakeWind),
            58 => Ok(WeatherSevereType::MarineWeather),
            59 => Ok(WeatherSevereType::Wind),
            60 => Ok(WeatherSevereType::SmallCraftHazardousSeas),
            61 => Ok(WeatherSevereType::HazardousSeas),
            62 => Ok(WeatherSevereType::SmallCraft),
            63 => Ok(WeatherSevereType::SmallCraftWinds),
            64 => Ok(WeatherSevereType::SmallCraftRoughBar),
            65 => Ok(WeatherSevereType::HighWaterLevel),
            66 => Ok(WeatherSevereType::Ashfall),
            67 => Ok(WeatherSevereType::FreezingFog),
            68 => Ok(WeatherSevereType::DenseFog),
            69 => Ok(WeatherSevereType::DenseSmoke),
            70 => Ok(WeatherSevereType::BlowingDust),
            71 => Ok(WeatherSevereType::HardFreeze),
            72 => Ok(WeatherSevereType::Freeze),
            73 => Ok(WeatherSevereType::Frost),
            74 => Ok(WeatherSevereType::FireWeather),
            75 => Ok(WeatherSevereType::Flood),
            76 => Ok(WeatherSevereType::RipTide),
            77 => Ok(WeatherSevereType::HighSurf),
            78 => Ok(WeatherSevereType::Smog),
            79 => Ok(WeatherSevereType::AirQuality),
            80 => Ok(WeatherSevereType::BriskWind),
            81 => Ok(WeatherSevereType::AirStagnation),
            82 => Ok(WeatherSevereType::LowWater),
            83 => Ok(WeatherSevereType::Hydrological),
            84 => Ok(WeatherSevereType::SpecialWeather),
            _ => Ok(WeatherSevereType::Unknown),
        }
    }
}
#[doc = "number of seconds into the day since 00:00:00 UTC"]
#[derive(Debug)]
pub struct TimeIntoDay(pub u32);
impl TimeIntoDay {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        Ok(TimeIntoDay(T::read_u32(buffer)))
    }
}
#[doc = "number of seconds into the day since local 00:00:00"]
#[derive(Debug)]
pub struct LocaltimeIntoDay(pub u32);
impl LocaltimeIntoDay {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        Ok(LocaltimeIntoDay(T::read_u32(buffer)))
    }
}
#[derive(Debug)]
pub enum StrokeType {
    NoEvent = 0,
    #[doc = "stroke was detected but cannot be identified"]
    Other = 1,
    Serve = 2,
    Forehand = 3,
    Backhand = 4,
    Smash = 5,
    Unknown,
}
impl StrokeType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(StrokeType::NoEvent),
            1 => Ok(StrokeType::Other),
            2 => Ok(StrokeType::Serve),
            3 => Ok(StrokeType::Forehand),
            4 => Ok(StrokeType::Backhand),
            5 => Ok(StrokeType::Smash),
            _ => Ok(StrokeType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum BodyLocation {
    LeftLeg = 0,
    LeftCalf = 1,
    LeftShin = 2,
    LeftHamstring = 3,
    LeftQuad = 4,
    LeftGlute = 5,
    RightLeg = 6,
    RightCalf = 7,
    RightShin = 8,
    RightHamstring = 9,
    RightQuad = 10,
    RightGlute = 11,
    TorsoBack = 12,
    LeftLowerBack = 13,
    LeftUpperBack = 14,
    RightLowerBack = 15,
    RightUpperBack = 16,
    TorsoFront = 17,
    LeftAbdomen = 18,
    LeftChest = 19,
    RightAbdomen = 20,
    RightChest = 21,
    LeftArm = 22,
    LeftShoulder = 23,
    LeftBicep = 24,
    LeftTricep = 25,
    #[doc = "Left anterior forearm"]
    LeftBrachioradialis = 26,
    #[doc = "Left posterior forearm"]
    LeftForearmExtensors = 27,
    RightArm = 28,
    RightShoulder = 29,
    RightBicep = 30,
    RightTricep = 31,
    #[doc = "Right anterior forearm"]
    RightBrachioradialis = 32,
    #[doc = "Right posterior forearm"]
    RightForearmExtensors = 33,
    Neck = 34,
    Throat = 35,
    WaistMidBack = 36,
    WaistFront = 37,
    WaistLeft = 38,
    WaistRight = 39,
    Unknown,
}
impl BodyLocation {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(BodyLocation::LeftLeg),
            1 => Ok(BodyLocation::LeftCalf),
            2 => Ok(BodyLocation::LeftShin),
            3 => Ok(BodyLocation::LeftHamstring),
            4 => Ok(BodyLocation::LeftQuad),
            5 => Ok(BodyLocation::LeftGlute),
            6 => Ok(BodyLocation::RightLeg),
            7 => Ok(BodyLocation::RightCalf),
            8 => Ok(BodyLocation::RightShin),
            9 => Ok(BodyLocation::RightHamstring),
            10 => Ok(BodyLocation::RightQuad),
            11 => Ok(BodyLocation::RightGlute),
            12 => Ok(BodyLocation::TorsoBack),
            13 => Ok(BodyLocation::LeftLowerBack),
            14 => Ok(BodyLocation::LeftUpperBack),
            15 => Ok(BodyLocation::RightLowerBack),
            16 => Ok(BodyLocation::RightUpperBack),
            17 => Ok(BodyLocation::TorsoFront),
            18 => Ok(BodyLocation::LeftAbdomen),
            19 => Ok(BodyLocation::LeftChest),
            20 => Ok(BodyLocation::RightAbdomen),
            21 => Ok(BodyLocation::RightChest),
            22 => Ok(BodyLocation::LeftArm),
            23 => Ok(BodyLocation::LeftShoulder),
            24 => Ok(BodyLocation::LeftBicep),
            25 => Ok(BodyLocation::LeftTricep),
            26 => Ok(BodyLocation::LeftBrachioradialis),
            27 => Ok(BodyLocation::LeftForearmExtensors),
            28 => Ok(BodyLocation::RightArm),
            29 => Ok(BodyLocation::RightShoulder),
            30 => Ok(BodyLocation::RightBicep),
            31 => Ok(BodyLocation::RightTricep),
            32 => Ok(BodyLocation::RightBrachioradialis),
            33 => Ok(BodyLocation::RightForearmExtensors),
            34 => Ok(BodyLocation::Neck),
            35 => Ok(BodyLocation::Throat),
            36 => Ok(BodyLocation::WaistMidBack),
            37 => Ok(BodyLocation::WaistFront),
            38 => Ok(BodyLocation::WaistLeft),
            39 => Ok(BodyLocation::WaistRight),
            _ => Ok(BodyLocation::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SegmentLapStatus {
    End = 0,
    Fail = 1,
    Unknown,
}
impl SegmentLapStatus {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SegmentLapStatus::End),
            1 => Ok(SegmentLapStatus::Fail),
            _ => Ok(SegmentLapStatus::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SegmentLeaderboardType {
    Overall = 0,
    PersonalBest = 1,
    Connections = 2,
    Group = 3,
    Challenger = 4,
    Kom = 5,
    Qom = 6,
    Pr = 7,
    Goal = 8,
    Rival = 9,
    ClubLeader = 10,
    Unknown,
}
impl SegmentLeaderboardType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SegmentLeaderboardType::Overall),
            1 => Ok(SegmentLeaderboardType::PersonalBest),
            2 => Ok(SegmentLeaderboardType::Connections),
            3 => Ok(SegmentLeaderboardType::Group),
            4 => Ok(SegmentLeaderboardType::Challenger),
            5 => Ok(SegmentLeaderboardType::Kom),
            6 => Ok(SegmentLeaderboardType::Qom),
            7 => Ok(SegmentLeaderboardType::Pr),
            8 => Ok(SegmentLeaderboardType::Goal),
            9 => Ok(SegmentLeaderboardType::Rival),
            10 => Ok(SegmentLeaderboardType::ClubLeader),
            _ => Ok(SegmentLeaderboardType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SegmentDeleteStatus {
    DoNotDelete = 0,
    DeleteOne = 1,
    DeleteAll = 2,
    Unknown,
}
impl SegmentDeleteStatus {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SegmentDeleteStatus::DoNotDelete),
            1 => Ok(SegmentDeleteStatus::DeleteOne),
            2 => Ok(SegmentDeleteStatus::DeleteAll),
            _ => Ok(SegmentDeleteStatus::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SegmentSelectionType {
    Starred = 0,
    Suggested = 1,
    Unknown,
}
impl SegmentSelectionType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SegmentSelectionType::Starred),
            1 => Ok(SegmentSelectionType::Suggested),
            _ => Ok(SegmentSelectionType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SourceType {
    #[doc = "External device connected with ANT"]
    Ant = 0,
    #[doc = "External device connected with ANT+"]
    Antplus = 1,
    #[doc = "External device connected with BT"]
    Bluetooth = 2,
    #[doc = "External device connected with BLE"]
    BluetoothLowEnergy = 3,
    #[doc = "External device connected with Wifi"]
    Wifi = 4,
    #[doc = "Onboard device"]
    Local = 5,
    Unknown,
}
impl SourceType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SourceType::Ant),
            1 => Ok(SourceType::Antplus),
            2 => Ok(SourceType::Bluetooth),
            3 => Ok(SourceType::BluetoothLowEnergy),
            4 => Ok(SourceType::Wifi),
            5 => Ok(SourceType::Local),
            _ => Ok(SourceType::Unknown),
        }
    }
}
#[derive(Debug)]
pub struct LocalDeviceType(pub u8);
impl LocalDeviceType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        Ok(LocalDeviceType(buffer[0]))
    }
}
#[derive(Debug)]
pub enum DisplayOrientation {
    #[doc = "automatic if the device supports it"]
    Auto = 0,
    Portrait = 1,
    Landscape = 2,
    #[doc = "portrait mode but rotated 180 degrees"]
    PortraitFlipped = 3,
    #[doc = "landscape mode but rotated 180 degrees"]
    LandscapeFlipped = 4,
    Unknown,
}
impl DisplayOrientation {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DisplayOrientation::Auto),
            1 => Ok(DisplayOrientation::Portrait),
            2 => Ok(DisplayOrientation::Landscape),
            3 => Ok(DisplayOrientation::PortraitFlipped),
            4 => Ok(DisplayOrientation::LandscapeFlipped),
            _ => Ok(DisplayOrientation::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum WorkoutEquipment {
    None = 0,
    SwimFins = 1,
    SwimKickboard = 2,
    SwimPaddles = 3,
    SwimPullBuoy = 4,
    SwimSnorkel = 5,
    Unknown,
}
impl WorkoutEquipment {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(WorkoutEquipment::None),
            1 => Ok(WorkoutEquipment::SwimFins),
            2 => Ok(WorkoutEquipment::SwimKickboard),
            3 => Ok(WorkoutEquipment::SwimPaddles),
            4 => Ok(WorkoutEquipment::SwimPullBuoy),
            5 => Ok(WorkoutEquipment::SwimSnorkel),
            _ => Ok(WorkoutEquipment::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum WatchfaceMode {
    Digital = 0,
    Analog = 1,
    ConnectIq = 2,
    Disabled = 3,
    Unknown,
}
impl WatchfaceMode {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(WatchfaceMode::Digital),
            1 => Ok(WatchfaceMode::Analog),
            2 => Ok(WatchfaceMode::ConnectIq),
            3 => Ok(WatchfaceMode::Disabled),
            _ => Ok(WatchfaceMode::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DigitalWatchfaceLayout {
    Traditional = 0,
    Modern = 1,
    Bold = 2,
    Unknown,
}
impl DigitalWatchfaceLayout {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DigitalWatchfaceLayout::Traditional),
            1 => Ok(DigitalWatchfaceLayout::Modern),
            2 => Ok(DigitalWatchfaceLayout::Bold),
            _ => Ok(DigitalWatchfaceLayout::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum AnalogWatchfaceLayout {
    Minimal = 0,
    Traditional = 1,
    Modern = 2,
    Unknown,
}
impl AnalogWatchfaceLayout {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(AnalogWatchfaceLayout::Minimal),
            1 => Ok(AnalogWatchfaceLayout::Traditional),
            2 => Ok(AnalogWatchfaceLayout::Modern),
            _ => Ok(AnalogWatchfaceLayout::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum RiderPositionType {
    Seated = 0,
    Standing = 1,
    TransitionToSeated = 2,
    TransitionToStanding = 3,
    Unknown,
}
impl RiderPositionType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(RiderPositionType::Seated),
            1 => Ok(RiderPositionType::Standing),
            2 => Ok(RiderPositionType::TransitionToSeated),
            3 => Ok(RiderPositionType::TransitionToStanding),
            _ => Ok(RiderPositionType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum PowerPhaseType {
    PowerPhaseStartAngle = 0,
    PowerPhaseEndAngle = 1,
    PowerPhaseArcLength = 2,
    PowerPhaseCenter = 3,
    Unknown,
}
impl PowerPhaseType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(PowerPhaseType::PowerPhaseStartAngle),
            1 => Ok(PowerPhaseType::PowerPhaseEndAngle),
            2 => Ok(PowerPhaseType::PowerPhaseArcLength),
            3 => Ok(PowerPhaseType::PowerPhaseCenter),
            _ => Ok(PowerPhaseType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum CameraEventType {
    #[doc = "Start of video recording"]
    VideoStart = 0,
    #[doc = "Mark of video file split (end of one file, beginning of the \
             other)"]
    VideoSplit = 1,
    #[doc = "End of video recording"]
    VideoEnd = 2,
    #[doc = "Still photo taken"]
    PhotoTaken = 3,
    VideoSecondStreamStart = 4,
    VideoSecondStreamSplit = 5,
    VideoSecondStreamEnd = 6,
    #[doc = "Mark of video file split start"]
    VideoSplitStart = 7,
    VideoSecondStreamSplitStart = 8,
    #[doc = "Mark when a video recording has been paused"]
    VideoPause = 11,
    VideoSecondStreamPause = 12,
    #[doc = "Mark when a video recording has been resumed"]
    VideoResume = 13,
    VideoSecondStreamResume = 14,
    Unknown,
}
impl CameraEventType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(CameraEventType::VideoStart),
            1 => Ok(CameraEventType::VideoSplit),
            2 => Ok(CameraEventType::VideoEnd),
            3 => Ok(CameraEventType::PhotoTaken),
            4 => Ok(CameraEventType::VideoSecondStreamStart),
            5 => Ok(CameraEventType::VideoSecondStreamSplit),
            6 => Ok(CameraEventType::VideoSecondStreamEnd),
            7 => Ok(CameraEventType::VideoSplitStart),
            8 => Ok(CameraEventType::VideoSecondStreamSplitStart),
            11 => Ok(CameraEventType::VideoPause),
            12 => Ok(CameraEventType::VideoSecondStreamPause),
            13 => Ok(CameraEventType::VideoResume),
            14 => Ok(CameraEventType::VideoSecondStreamResume),
            _ => Ok(CameraEventType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SensorType {
    Accelerometer = 0,
    Gyroscope = 1,
    #[doc = "Magnetometer"]
    Compass = 2,
    Barometer = 3,
    Unknown,
}
impl SensorType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SensorType::Accelerometer),
            1 => Ok(SensorType::Gyroscope),
            2 => Ok(SensorType::Compass),
            3 => Ok(SensorType::Barometer),
            _ => Ok(SensorType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum BikeLightNetworkConfigType {
    Auto = 0,
    Individual = 4,
    HighVisibility = 5,
    Trail = 6,
    Unknown,
}
impl BikeLightNetworkConfigType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(BikeLightNetworkConfigType::Auto),
            4 => Ok(BikeLightNetworkConfigType::Individual),
            5 => Ok(BikeLightNetworkConfigType::HighVisibility),
            6 => Ok(BikeLightNetworkConfigType::Trail),
            _ => Ok(BikeLightNetworkConfigType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum CommTimeoutType {
    #[doc = "Timeout pairing to any device"]
    WildcardPairingTimeout = 0,
    #[doc = "Timeout pairing to previously paired device"]
    PairingTimeout = 1,
    #[doc = "Temporary loss of communications"]
    ConnectionLost = 2,
    #[doc = "Connection closed due to extended bad communications"]
    ConnectionTimeout = 3,
    Unknown,
}
impl CommTimeoutType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(CommTimeoutType::WildcardPairingTimeout),
            1 => Ok(CommTimeoutType::PairingTimeout),
            2 => Ok(CommTimeoutType::ConnectionLost),
            3 => Ok(CommTimeoutType::ConnectionTimeout),
            _ => Ok(CommTimeoutType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum CameraOrientationType {
    CameraOrientation0 = 0,
    CameraOrientation90 = 1,
    CameraOrientation180 = 2,
    CameraOrientation270 = 3,
    Unknown,
}
impl CameraOrientationType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(CameraOrientationType::CameraOrientation0),
            1 => Ok(CameraOrientationType::CameraOrientation90),
            2 => Ok(CameraOrientationType::CameraOrientation180),
            3 => Ok(CameraOrientationType::CameraOrientation270),
            _ => Ok(CameraOrientationType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum AttitudeStage {
    Failed = 0,
    Aligning = 1,
    Degraded = 2,
    Valid = 3,
    Unknown,
}
impl AttitudeStage {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(AttitudeStage::Failed),
            1 => Ok(AttitudeStage::Aligning),
            2 => Ok(AttitudeStage::Degraded),
            3 => Ok(AttitudeStage::Valid),
            _ => Ok(AttitudeStage::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum AttitudeValidity {
    TrackAngleHeadingValid = 1,
    PitchValid = 2,
    RollValid = 4,
    LateralBodyAccelValid = 8,
    NormalBodyAccelValid = 16,
    TurnRateValid = 32,
    HwFail = 64,
    MagInvalid = 128,
    NoGps = 256,
    GpsInvalid = 512,
    SolutionCoasting = 1024,
    TrueTrackAngle = 2048,
    MagneticHeading = 4096,
    Unknown,
}
impl AttitudeValidity {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(AttitudeValidity::TrackAngleHeadingValid),
            2 => Ok(AttitudeValidity::PitchValid),
            4 => Ok(AttitudeValidity::RollValid),
            8 => Ok(AttitudeValidity::LateralBodyAccelValid),
            16 => Ok(AttitudeValidity::NormalBodyAccelValid),
            32 => Ok(AttitudeValidity::TurnRateValid),
            64 => Ok(AttitudeValidity::HwFail),
            128 => Ok(AttitudeValidity::MagInvalid),
            256 => Ok(AttitudeValidity::NoGps),
            512 => Ok(AttitudeValidity::GpsInvalid),
            1024 => Ok(AttitudeValidity::SolutionCoasting),
            2048 => Ok(AttitudeValidity::TrueTrackAngle),
            4096 => Ok(AttitudeValidity::MagneticHeading),
            _ => Ok(AttitudeValidity::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum AutoSyncFrequency {
    Never = 0,
    Occasionally = 1,
    Frequent = 2,
    OnceADay = 3,
    Remote = 4,
    Unknown,
}
impl AutoSyncFrequency {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(AutoSyncFrequency::Never),
            1 => Ok(AutoSyncFrequency::Occasionally),
            2 => Ok(AutoSyncFrequency::Frequent),
            3 => Ok(AutoSyncFrequency::OnceADay),
            4 => Ok(AutoSyncFrequency::Remote),
            _ => Ok(AutoSyncFrequency::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ExdLayout {
    FullScreen = 0,
    HalfVertical = 1,
    HalfHorizontal = 2,
    HalfVerticalRightSplit = 3,
    HalfHorizontalBottomSplit = 4,
    FullQuarterSplit = 5,
    HalfVerticalLeftSplit = 6,
    HalfHorizontalTopSplit = 7,
    Unknown,
}
impl ExdLayout {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ExdLayout::FullScreen),
            1 => Ok(ExdLayout::HalfVertical),
            2 => Ok(ExdLayout::HalfHorizontal),
            3 => Ok(ExdLayout::HalfVerticalRightSplit),
            4 => Ok(ExdLayout::HalfHorizontalBottomSplit),
            5 => Ok(ExdLayout::FullQuarterSplit),
            6 => Ok(ExdLayout::HalfVerticalLeftSplit),
            7 => Ok(ExdLayout::HalfHorizontalTopSplit),
            _ => Ok(ExdLayout::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ExdDisplayType {
    Numerical = 0,
    Simple = 1,
    Graph = 2,
    Bar = 3,
    CircleGraph = 4,
    VirtualPartner = 5,
    Balance = 6,
    StringList = 7,
    String = 8,
    SimpleDynamicIcon = 9,
    Gauge = 10,
    Unknown,
}
impl ExdDisplayType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ExdDisplayType::Numerical),
            1 => Ok(ExdDisplayType::Simple),
            2 => Ok(ExdDisplayType::Graph),
            3 => Ok(ExdDisplayType::Bar),
            4 => Ok(ExdDisplayType::CircleGraph),
            5 => Ok(ExdDisplayType::VirtualPartner),
            6 => Ok(ExdDisplayType::Balance),
            7 => Ok(ExdDisplayType::StringList),
            8 => Ok(ExdDisplayType::String),
            9 => Ok(ExdDisplayType::SimpleDynamicIcon),
            10 => Ok(ExdDisplayType::Gauge),
            _ => Ok(ExdDisplayType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ExdDataUnits {
    NoUnits = 0,
    Laps = 1,
    MilesPerHour = 2,
    KilometersPerHour = 3,
    FeetPerHour = 4,
    MetersPerHour = 5,
    DegreesCelsius = 6,
    DegreesFarenheit = 7,
    Zone = 8,
    Gear = 9,
    Rpm = 10,
    Bpm = 11,
    Degrees = 12,
    Millimeters = 13,
    Meters = 14,
    Kilometers = 15,
    Feet = 16,
    Yards = 17,
    Kilofeet = 18,
    Miles = 19,
    Time = 20,
    EnumTurnType = 21,
    Percent = 22,
    Watts = 23,
    WattsPerKilogram = 24,
    EnumBatteryStatus = 25,
    EnumBikeLightBeamAngleMode = 26,
    EnumBikeLightBatteryStatus = 27,
    EnumBikeLightNetworkConfigType = 28,
    Lights = 29,
    Seconds = 30,
    Minutes = 31,
    Hours = 32,
    Calories = 33,
    Kilojoules = 34,
    Milliseconds = 35,
    SecondPerMile = 36,
    SecondPerKilometer = 37,
    Centimeter = 38,
    EnumCoursePoint = 39,
    Bradians = 40,
    EnumSport = 41,
    InchesHg = 42,
    MmHg = 43,
    Mbars = 44,
    HectoPascals = 45,
    FeetPerMin = 46,
    MetersPerMin = 47,
    MetersPerSec = 48,
    EightCardinal = 49,
    Unknown,
}
impl ExdDataUnits {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ExdDataUnits::NoUnits),
            1 => Ok(ExdDataUnits::Laps),
            2 => Ok(ExdDataUnits::MilesPerHour),
            3 => Ok(ExdDataUnits::KilometersPerHour),
            4 => Ok(ExdDataUnits::FeetPerHour),
            5 => Ok(ExdDataUnits::MetersPerHour),
            6 => Ok(ExdDataUnits::DegreesCelsius),
            7 => Ok(ExdDataUnits::DegreesFarenheit),
            8 => Ok(ExdDataUnits::Zone),
            9 => Ok(ExdDataUnits::Gear),
            10 => Ok(ExdDataUnits::Rpm),
            11 => Ok(ExdDataUnits::Bpm),
            12 => Ok(ExdDataUnits::Degrees),
            13 => Ok(ExdDataUnits::Millimeters),
            14 => Ok(ExdDataUnits::Meters),
            15 => Ok(ExdDataUnits::Kilometers),
            16 => Ok(ExdDataUnits::Feet),
            17 => Ok(ExdDataUnits::Yards),
            18 => Ok(ExdDataUnits::Kilofeet),
            19 => Ok(ExdDataUnits::Miles),
            20 => Ok(ExdDataUnits::Time),
            21 => Ok(ExdDataUnits::EnumTurnType),
            22 => Ok(ExdDataUnits::Percent),
            23 => Ok(ExdDataUnits::Watts),
            24 => Ok(ExdDataUnits::WattsPerKilogram),
            25 => Ok(ExdDataUnits::EnumBatteryStatus),
            26 => Ok(ExdDataUnits::EnumBikeLightBeamAngleMode),
            27 => Ok(ExdDataUnits::EnumBikeLightBatteryStatus),
            28 => Ok(ExdDataUnits::EnumBikeLightNetworkConfigType),
            29 => Ok(ExdDataUnits::Lights),
            30 => Ok(ExdDataUnits::Seconds),
            31 => Ok(ExdDataUnits::Minutes),
            32 => Ok(ExdDataUnits::Hours),
            33 => Ok(ExdDataUnits::Calories),
            34 => Ok(ExdDataUnits::Kilojoules),
            35 => Ok(ExdDataUnits::Milliseconds),
            36 => Ok(ExdDataUnits::SecondPerMile),
            37 => Ok(ExdDataUnits::SecondPerKilometer),
            38 => Ok(ExdDataUnits::Centimeter),
            39 => Ok(ExdDataUnits::EnumCoursePoint),
            40 => Ok(ExdDataUnits::Bradians),
            41 => Ok(ExdDataUnits::EnumSport),
            42 => Ok(ExdDataUnits::InchesHg),
            43 => Ok(ExdDataUnits::MmHg),
            44 => Ok(ExdDataUnits::Mbars),
            45 => Ok(ExdDataUnits::HectoPascals),
            46 => Ok(ExdDataUnits::FeetPerMin),
            47 => Ok(ExdDataUnits::MetersPerMin),
            48 => Ok(ExdDataUnits::MetersPerSec),
            49 => Ok(ExdDataUnits::EightCardinal),
            _ => Ok(ExdDataUnits::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ExdQualifiers {
    NoQualifier = 0,
    Instantaneous = 1,
    Average = 2,
    Lap = 3,
    Maximum = 4,
    MaximumAverage = 5,
    MaximumLap = 6,
    LastLap = 7,
    AverageLap = 8,
    ToDestination = 9,
    ToGo = 10,
    ToNext = 11,
    NextCoursePoint = 12,
    Total = 13,
    ThreeSecondAverage = 14,
    TenSecondAverage = 15,
    ThirtySecondAverage = 16,
    PercentMaximum = 17,
    PercentMaximumAverage = 18,
    LapPercentMaximum = 19,
    Elapsed = 20,
    Sunrise = 21,
    Sunset = 22,
    ComparedToVirtualPartner = 23,
    Maximum24H = 24,
    Minimum24H = 25,
    Minimum = 26,
    First = 27,
    Second = 28,
    Third = 29,
    Shifter = 30,
    LastSport = 31,
    Moving = 32,
    Stopped = 33,
    EstimatedTotal = 34,
    Zone9 = 242,
    Zone8 = 243,
    Zone7 = 244,
    Zone6 = 245,
    Zone5 = 246,
    Zone4 = 247,
    Zone3 = 248,
    Zone2 = 249,
    Zone1 = 250,
    Unknown,
}
impl ExdQualifiers {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ExdQualifiers::NoQualifier),
            1 => Ok(ExdQualifiers::Instantaneous),
            2 => Ok(ExdQualifiers::Average),
            3 => Ok(ExdQualifiers::Lap),
            4 => Ok(ExdQualifiers::Maximum),
            5 => Ok(ExdQualifiers::MaximumAverage),
            6 => Ok(ExdQualifiers::MaximumLap),
            7 => Ok(ExdQualifiers::LastLap),
            8 => Ok(ExdQualifiers::AverageLap),
            9 => Ok(ExdQualifiers::ToDestination),
            10 => Ok(ExdQualifiers::ToGo),
            11 => Ok(ExdQualifiers::ToNext),
            12 => Ok(ExdQualifiers::NextCoursePoint),
            13 => Ok(ExdQualifiers::Total),
            14 => Ok(ExdQualifiers::ThreeSecondAverage),
            15 => Ok(ExdQualifiers::TenSecondAverage),
            16 => Ok(ExdQualifiers::ThirtySecondAverage),
            17 => Ok(ExdQualifiers::PercentMaximum),
            18 => Ok(ExdQualifiers::PercentMaximumAverage),
            19 => Ok(ExdQualifiers::LapPercentMaximum),
            20 => Ok(ExdQualifiers::Elapsed),
            21 => Ok(ExdQualifiers::Sunrise),
            22 => Ok(ExdQualifiers::Sunset),
            23 => Ok(ExdQualifiers::ComparedToVirtualPartner),
            24 => Ok(ExdQualifiers::Maximum24H),
            25 => Ok(ExdQualifiers::Minimum24H),
            26 => Ok(ExdQualifiers::Minimum),
            27 => Ok(ExdQualifiers::First),
            28 => Ok(ExdQualifiers::Second),
            29 => Ok(ExdQualifiers::Third),
            30 => Ok(ExdQualifiers::Shifter),
            31 => Ok(ExdQualifiers::LastSport),
            32 => Ok(ExdQualifiers::Moving),
            33 => Ok(ExdQualifiers::Stopped),
            34 => Ok(ExdQualifiers::EstimatedTotal),
            242 => Ok(ExdQualifiers::Zone9),
            243 => Ok(ExdQualifiers::Zone8),
            244 => Ok(ExdQualifiers::Zone7),
            245 => Ok(ExdQualifiers::Zone6),
            246 => Ok(ExdQualifiers::Zone5),
            247 => Ok(ExdQualifiers::Zone4),
            248 => Ok(ExdQualifiers::Zone3),
            249 => Ok(ExdQualifiers::Zone2),
            250 => Ok(ExdQualifiers::Zone1),
            _ => Ok(ExdQualifiers::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ExdDescriptors {
    BikeLightBatteryStatus = 0,
    BeamAngleStatus = 1,
    BateryLevel = 2,
    LightNetworkMode = 3,
    NumberLightsConnected = 4,
    Cadence = 5,
    Distance = 6,
    EstimatedTimeOfArrival = 7,
    Heading = 8,
    Time = 9,
    BatteryLevel = 10,
    TrainerResistance = 11,
    TrainerTargetPower = 12,
    TimeSeated = 13,
    TimeStanding = 14,
    Elevation = 15,
    Grade = 16,
    Ascent = 17,
    Descent = 18,
    VerticalSpeed = 19,
    Di2BatteryLevel = 20,
    FrontGear = 21,
    RearGear = 22,
    GearRatio = 23,
    HeartRate = 24,
    HeartRateZone = 25,
    TimeInHeartRateZone = 26,
    HeartRateReserve = 27,
    Calories = 28,
    GpsAccuracy = 29,
    GpsSignalStrength = 30,
    Temperature = 31,
    TimeOfDay = 32,
    Balance = 33,
    PedalSmoothness = 34,
    Power = 35,
    FunctionalThresholdPower = 36,
    IntensityFactor = 37,
    Work = 38,
    PowerRatio = 39,
    NormalizedPower = 40,
    TrainingStressScore = 41,
    TimeOnZone = 42,
    Speed = 43,
    Laps = 44,
    Reps = 45,
    WorkoutStep = 46,
    CourseDistance = 47,
    NavigationDistance = 48,
    CourseEstimatedTimeOfArrival = 49,
    NavigationEstimatedTimeOfArrival = 50,
    CourseTime = 51,
    NavigationTime = 52,
    CourseHeading = 53,
    NavigationHeading = 54,
    PowerZone = 55,
    TorqueEffectiveness = 56,
    TimerTime = 57,
    PowerWeightRatio = 58,
    LeftPlatformCenterOffset = 59,
    RightPlatformCenterOffset = 60,
    LeftPowerPhaseStartAngle = 61,
    RightPowerPhaseStartAngle = 62,
    LeftPowerPhaseFinishAngle = 63,
    RightPowerPhaseFinishAngle = 64,
    #[doc = "Combined gear information"]
    Gears = 65,
    Pace = 66,
    TrainingEffect = 67,
    VerticalOscillation = 68,
    VerticalRatio = 69,
    GroundContactTime = 70,
    LeftGroundContactTimeBalance = 71,
    RightGroundContactTimeBalance = 72,
    StrideLength = 73,
    RunningCadence = 74,
    PerformanceCondition = 75,
    CourseType = 76,
    TimeInPowerZone = 77,
    NavigationTurn = 78,
    CourseLocation = 79,
    NavigationLocation = 80,
    Compass = 81,
    GearCombo = 82,
    MuscleOxygen = 83,
    Icon = 84,
    CompassHeading = 85,
    GpsHeading = 86,
    GpsElevation = 87,
    AnaerobicTrainingEffect = 88,
    Course = 89,
    OffCourse = 90,
    GlideRatio = 91,
    VerticalDistance = 92,
    Vmg = 93,
    AmbientPressure = 94,
    Pressure = 95,
    Vam = 96,
    Unknown,
}
impl ExdDescriptors {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ExdDescriptors::BikeLightBatteryStatus),
            1 => Ok(ExdDescriptors::BeamAngleStatus),
            2 => Ok(ExdDescriptors::BateryLevel),
            3 => Ok(ExdDescriptors::LightNetworkMode),
            4 => Ok(ExdDescriptors::NumberLightsConnected),
            5 => Ok(ExdDescriptors::Cadence),
            6 => Ok(ExdDescriptors::Distance),
            7 => Ok(ExdDescriptors::EstimatedTimeOfArrival),
            8 => Ok(ExdDescriptors::Heading),
            9 => Ok(ExdDescriptors::Time),
            10 => Ok(ExdDescriptors::BatteryLevel),
            11 => Ok(ExdDescriptors::TrainerResistance),
            12 => Ok(ExdDescriptors::TrainerTargetPower),
            13 => Ok(ExdDescriptors::TimeSeated),
            14 => Ok(ExdDescriptors::TimeStanding),
            15 => Ok(ExdDescriptors::Elevation),
            16 => Ok(ExdDescriptors::Grade),
            17 => Ok(ExdDescriptors::Ascent),
            18 => Ok(ExdDescriptors::Descent),
            19 => Ok(ExdDescriptors::VerticalSpeed),
            20 => Ok(ExdDescriptors::Di2BatteryLevel),
            21 => Ok(ExdDescriptors::FrontGear),
            22 => Ok(ExdDescriptors::RearGear),
            23 => Ok(ExdDescriptors::GearRatio),
            24 => Ok(ExdDescriptors::HeartRate),
            25 => Ok(ExdDescriptors::HeartRateZone),
            26 => Ok(ExdDescriptors::TimeInHeartRateZone),
            27 => Ok(ExdDescriptors::HeartRateReserve),
            28 => Ok(ExdDescriptors::Calories),
            29 => Ok(ExdDescriptors::GpsAccuracy),
            30 => Ok(ExdDescriptors::GpsSignalStrength),
            31 => Ok(ExdDescriptors::Temperature),
            32 => Ok(ExdDescriptors::TimeOfDay),
            33 => Ok(ExdDescriptors::Balance),
            34 => Ok(ExdDescriptors::PedalSmoothness),
            35 => Ok(ExdDescriptors::Power),
            36 => Ok(ExdDescriptors::FunctionalThresholdPower),
            37 => Ok(ExdDescriptors::IntensityFactor),
            38 => Ok(ExdDescriptors::Work),
            39 => Ok(ExdDescriptors::PowerRatio),
            40 => Ok(ExdDescriptors::NormalizedPower),
            41 => Ok(ExdDescriptors::TrainingStressScore),
            42 => Ok(ExdDescriptors::TimeOnZone),
            43 => Ok(ExdDescriptors::Speed),
            44 => Ok(ExdDescriptors::Laps),
            45 => Ok(ExdDescriptors::Reps),
            46 => Ok(ExdDescriptors::WorkoutStep),
            47 => Ok(ExdDescriptors::CourseDistance),
            48 => Ok(ExdDescriptors::NavigationDistance),
            49 => Ok(ExdDescriptors::CourseEstimatedTimeOfArrival),
            50 => Ok(ExdDescriptors::NavigationEstimatedTimeOfArrival),
            51 => Ok(ExdDescriptors::CourseTime),
            52 => Ok(ExdDescriptors::NavigationTime),
            53 => Ok(ExdDescriptors::CourseHeading),
            54 => Ok(ExdDescriptors::NavigationHeading),
            55 => Ok(ExdDescriptors::PowerZone),
            56 => Ok(ExdDescriptors::TorqueEffectiveness),
            57 => Ok(ExdDescriptors::TimerTime),
            58 => Ok(ExdDescriptors::PowerWeightRatio),
            59 => Ok(ExdDescriptors::LeftPlatformCenterOffset),
            60 => Ok(ExdDescriptors::RightPlatformCenterOffset),
            61 => Ok(ExdDescriptors::LeftPowerPhaseStartAngle),
            62 => Ok(ExdDescriptors::RightPowerPhaseStartAngle),
            63 => Ok(ExdDescriptors::LeftPowerPhaseFinishAngle),
            64 => Ok(ExdDescriptors::RightPowerPhaseFinishAngle),
            65 => Ok(ExdDescriptors::Gears),
            66 => Ok(ExdDescriptors::Pace),
            67 => Ok(ExdDescriptors::TrainingEffect),
            68 => Ok(ExdDescriptors::VerticalOscillation),
            69 => Ok(ExdDescriptors::VerticalRatio),
            70 => Ok(ExdDescriptors::GroundContactTime),
            71 => Ok(ExdDescriptors::LeftGroundContactTimeBalance),
            72 => Ok(ExdDescriptors::RightGroundContactTimeBalance),
            73 => Ok(ExdDescriptors::StrideLength),
            74 => Ok(ExdDescriptors::RunningCadence),
            75 => Ok(ExdDescriptors::PerformanceCondition),
            76 => Ok(ExdDescriptors::CourseType),
            77 => Ok(ExdDescriptors::TimeInPowerZone),
            78 => Ok(ExdDescriptors::NavigationTurn),
            79 => Ok(ExdDescriptors::CourseLocation),
            80 => Ok(ExdDescriptors::NavigationLocation),
            81 => Ok(ExdDescriptors::Compass),
            82 => Ok(ExdDescriptors::GearCombo),
            83 => Ok(ExdDescriptors::MuscleOxygen),
            84 => Ok(ExdDescriptors::Icon),
            85 => Ok(ExdDescriptors::CompassHeading),
            86 => Ok(ExdDescriptors::GpsHeading),
            87 => Ok(ExdDescriptors::GpsElevation),
            88 => Ok(ExdDescriptors::AnaerobicTrainingEffect),
            89 => Ok(ExdDescriptors::Course),
            90 => Ok(ExdDescriptors::OffCourse),
            91 => Ok(ExdDescriptors::GlideRatio),
            92 => Ok(ExdDescriptors::VerticalDistance),
            93 => Ok(ExdDescriptors::Vmg),
            94 => Ok(ExdDescriptors::AmbientPressure),
            95 => Ok(ExdDescriptors::Pressure),
            96 => Ok(ExdDescriptors::Vam),
            _ => Ok(ExdDescriptors::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum AutoActivityDetect {
    None = 0,
    Running = 1,
    Cycling = 2,
    Swimming = 4,
    Walking = 8,
    Elliptical = 32,
    Sedentary = 1024,
    Unknown,
}
impl AutoActivityDetect {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint32::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(AutoActivityDetect::None),
            1 => Ok(AutoActivityDetect::Running),
            2 => Ok(AutoActivityDetect::Cycling),
            4 => Ok(AutoActivityDetect::Swimming),
            8 => Ok(AutoActivityDetect::Walking),
            32 => Ok(AutoActivityDetect::Elliptical),
            1024 => Ok(AutoActivityDetect::Sedentary),
            _ => Ok(AutoActivityDetect::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SupportedExdScreenLayouts {
    FullScreen = 1,
    HalfVertical = 2,
    HalfHorizontal = 4,
    HalfVerticalRightSplit = 8,
    HalfHorizontalBottomSplit = 16,
    FullQuarterSplit = 32,
    HalfVerticalLeftSplit = 64,
    HalfHorizontalTopSplit = 128,
    Unknown,
}
impl SupportedExdScreenLayouts {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint32z::decode::<T>(buffer)?;
        match base_value.0 {
            1 => Ok(SupportedExdScreenLayouts::FullScreen),
            2 => Ok(SupportedExdScreenLayouts::HalfVertical),
            4 => Ok(SupportedExdScreenLayouts::HalfHorizontal),
            8 => Ok(SupportedExdScreenLayouts::HalfVerticalRightSplit),
            16 => Ok(SupportedExdScreenLayouts::HalfHorizontalBottomSplit),
            32 => Ok(SupportedExdScreenLayouts::FullQuarterSplit),
            64 => Ok(SupportedExdScreenLayouts::HalfVerticalLeftSplit),
            128 => Ok(SupportedExdScreenLayouts::HalfHorizontalTopSplit),
            _ => Ok(SupportedExdScreenLayouts::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum FitBaseType {
    Enum = 0,
    Sint8 = 1,
    Uint8 = 2,
    Sint16 = 131,
    Uint16 = 132,
    Sint32 = 133,
    Uint32 = 134,
    String = 7,
    Float32 = 136,
    Float64 = 137,
    Uint8Z = 10,
    Uint16Z = 139,
    Uint32Z = 140,
    Byte = 13,
    Sint64 = 142,
    Uint64 = 143,
    Uint64Z = 144,
    Unknown,
}
impl FitBaseType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(FitBaseType::Enum),
            1 => Ok(FitBaseType::Sint8),
            2 => Ok(FitBaseType::Uint8),
            131 => Ok(FitBaseType::Sint16),
            132 => Ok(FitBaseType::Uint16),
            133 => Ok(FitBaseType::Sint32),
            134 => Ok(FitBaseType::Uint32),
            7 => Ok(FitBaseType::String),
            136 => Ok(FitBaseType::Float32),
            137 => Ok(FitBaseType::Float64),
            10 => Ok(FitBaseType::Uint8Z),
            139 => Ok(FitBaseType::Uint16Z),
            140 => Ok(FitBaseType::Uint32Z),
            13 => Ok(FitBaseType::Byte),
            142 => Ok(FitBaseType::Sint64),
            143 => Ok(FitBaseType::Uint64),
            144 => Ok(FitBaseType::Uint64Z),
            _ => Ok(FitBaseType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum TurnType {
    ArrivingIdx = 0,
    ArrivingLeftIdx = 1,
    ArrivingRightIdx = 2,
    ArrivingViaIdx = 3,
    ArrivingViaLeftIdx = 4,
    ArrivingViaRightIdx = 5,
    BearKeepLeftIdx = 6,
    BearKeepRightIdx = 7,
    ContinueIdx = 8,
    ExitLeftIdx = 9,
    ExitRightIdx = 10,
    FerryIdx = 11,
    Roundabout45Idx = 12,
    Roundabout90Idx = 13,
    Roundabout135Idx = 14,
    Roundabout180Idx = 15,
    Roundabout225Idx = 16,
    Roundabout270Idx = 17,
    Roundabout315Idx = 18,
    Roundabout360Idx = 19,
    RoundaboutNeg45Idx = 20,
    RoundaboutNeg90Idx = 21,
    RoundaboutNeg135Idx = 22,
    RoundaboutNeg180Idx = 23,
    RoundaboutNeg225Idx = 24,
    RoundaboutNeg270Idx = 25,
    RoundaboutNeg315Idx = 26,
    RoundaboutNeg360Idx = 27,
    RoundaboutGenericIdx = 28,
    RoundaboutNegGenericIdx = 29,
    SharpTurnLeftIdx = 30,
    SharpTurnRightIdx = 31,
    TurnLeftIdx = 32,
    TurnRightIdx = 33,
    UturnLeftIdx = 34,
    UturnRightIdx = 35,
    IconInvIdx = 36,
    IconIdxCnt = 37,
    Unknown,
}
impl TurnType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(TurnType::ArrivingIdx),
            1 => Ok(TurnType::ArrivingLeftIdx),
            2 => Ok(TurnType::ArrivingRightIdx),
            3 => Ok(TurnType::ArrivingViaIdx),
            4 => Ok(TurnType::ArrivingViaLeftIdx),
            5 => Ok(TurnType::ArrivingViaRightIdx),
            6 => Ok(TurnType::BearKeepLeftIdx),
            7 => Ok(TurnType::BearKeepRightIdx),
            8 => Ok(TurnType::ContinueIdx),
            9 => Ok(TurnType::ExitLeftIdx),
            10 => Ok(TurnType::ExitRightIdx),
            11 => Ok(TurnType::FerryIdx),
            12 => Ok(TurnType::Roundabout45Idx),
            13 => Ok(TurnType::Roundabout90Idx),
            14 => Ok(TurnType::Roundabout135Idx),
            15 => Ok(TurnType::Roundabout180Idx),
            16 => Ok(TurnType::Roundabout225Idx),
            17 => Ok(TurnType::Roundabout270Idx),
            18 => Ok(TurnType::Roundabout315Idx),
            19 => Ok(TurnType::Roundabout360Idx),
            20 => Ok(TurnType::RoundaboutNeg45Idx),
            21 => Ok(TurnType::RoundaboutNeg90Idx),
            22 => Ok(TurnType::RoundaboutNeg135Idx),
            23 => Ok(TurnType::RoundaboutNeg180Idx),
            24 => Ok(TurnType::RoundaboutNeg225Idx),
            25 => Ok(TurnType::RoundaboutNeg270Idx),
            26 => Ok(TurnType::RoundaboutNeg315Idx),
            27 => Ok(TurnType::RoundaboutNeg360Idx),
            28 => Ok(TurnType::RoundaboutGenericIdx),
            29 => Ok(TurnType::RoundaboutNegGenericIdx),
            30 => Ok(TurnType::SharpTurnLeftIdx),
            31 => Ok(TurnType::SharpTurnRightIdx),
            32 => Ok(TurnType::TurnLeftIdx),
            33 => Ok(TurnType::TurnRightIdx),
            34 => Ok(TurnType::UturnLeftIdx),
            35 => Ok(TurnType::UturnRightIdx),
            36 => Ok(TurnType::IconInvIdx),
            37 => Ok(TurnType::IconIdxCnt),
            _ => Ok(TurnType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum BikeLightBeamAngleMode {
    Manual = 0,
    Auto = 1,
    Unknown,
}
impl BikeLightBeamAngleMode {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(BikeLightBeamAngleMode::Manual),
            1 => Ok(BikeLightBeamAngleMode::Auto),
            _ => Ok(BikeLightBeamAngleMode::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum FitBaseUnit {
    Other = 0,
    Kilogram = 1,
    Pound = 2,
    Unknown,
}
impl FitBaseUnit {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(FitBaseUnit::Other),
            1 => Ok(FitBaseUnit::Kilogram),
            2 => Ok(FitBaseUnit::Pound),
            _ => Ok(FitBaseUnit::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SetType {
    Rest = 0,
    Active = 1,
    Unknown,
}
impl SetType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint8::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SetType::Rest),
            1 => Ok(SetType::Active),
            _ => Ok(SetType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ExerciseCategory {
    BenchPress = 0,
    CalfRaise = 1,
    Cardio = 2,
    Carry = 3,
    Chop = 4,
    Core = 5,
    Crunch = 6,
    Curl = 7,
    Deadlift = 8,
    Flye = 9,
    HipRaise = 10,
    HipStability = 11,
    HipSwing = 12,
    Hyperextension = 13,
    LateralRaise = 14,
    LegCurl = 15,
    LegRaise = 16,
    Lunge = 17,
    OlympicLift = 18,
    Plank = 19,
    Plyo = 20,
    PullUp = 21,
    PushUp = 22,
    Row = 23,
    ShoulderPress = 24,
    ShoulderStability = 25,
    Shrug = 26,
    SitUp = 27,
    Squat = 28,
    TotalBody = 29,
    TricepsExtension = 30,
    WarmUp = 31,
    Run = 32,
    Unknown,
}
impl ExerciseCategory {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ExerciseCategory::BenchPress),
            1 => Ok(ExerciseCategory::CalfRaise),
            2 => Ok(ExerciseCategory::Cardio),
            3 => Ok(ExerciseCategory::Carry),
            4 => Ok(ExerciseCategory::Chop),
            5 => Ok(ExerciseCategory::Core),
            6 => Ok(ExerciseCategory::Crunch),
            7 => Ok(ExerciseCategory::Curl),
            8 => Ok(ExerciseCategory::Deadlift),
            9 => Ok(ExerciseCategory::Flye),
            10 => Ok(ExerciseCategory::HipRaise),
            11 => Ok(ExerciseCategory::HipStability),
            12 => Ok(ExerciseCategory::HipSwing),
            13 => Ok(ExerciseCategory::Hyperextension),
            14 => Ok(ExerciseCategory::LateralRaise),
            15 => Ok(ExerciseCategory::LegCurl),
            16 => Ok(ExerciseCategory::LegRaise),
            17 => Ok(ExerciseCategory::Lunge),
            18 => Ok(ExerciseCategory::OlympicLift),
            19 => Ok(ExerciseCategory::Plank),
            20 => Ok(ExerciseCategory::Plyo),
            21 => Ok(ExerciseCategory::PullUp),
            22 => Ok(ExerciseCategory::PushUp),
            23 => Ok(ExerciseCategory::Row),
            24 => Ok(ExerciseCategory::ShoulderPress),
            25 => Ok(ExerciseCategory::ShoulderStability),
            26 => Ok(ExerciseCategory::Shrug),
            27 => Ok(ExerciseCategory::SitUp),
            28 => Ok(ExerciseCategory::Squat),
            29 => Ok(ExerciseCategory::TotalBody),
            30 => Ok(ExerciseCategory::TricepsExtension),
            31 => Ok(ExerciseCategory::WarmUp),
            32 => Ok(ExerciseCategory::Run),
            65534 => Ok(ExerciseCategory::Unknown),
            _ => Ok(ExerciseCategory::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum BenchPressExerciseName {
    AlternatingDumbbellChestPressOnSwissBall = 0,
    BarbellBenchPress = 1,
    BarbellBoardBenchPress = 2,
    BarbellFloorPress = 3,
    CloseGripBarbellBenchPress = 4,
    DeclineDumbbellBenchPress = 5,
    DumbbellBenchPress = 6,
    DumbbellFloorPress = 7,
    InclineBarbellBenchPress = 8,
    InclineDumbbellBenchPress = 9,
    InclineSmithMachineBenchPress = 10,
    IsometricBarbellBenchPress = 11,
    KettlebellChestPress = 12,
    NeutralGripDumbbellBenchPress = 13,
    NeutralGripDumbbellInclineBenchPress = 14,
    OneArmFloorPress = 15,
    WeightedOneArmFloorPress = 16,
    PartialLockout = 17,
    ReverseGripBarbellBenchPress = 18,
    ReverseGripInclineBenchPress = 19,
    SingleArmCableChestPress = 20,
    SingleArmDumbbellBenchPress = 21,
    SmithMachineBenchPress = 22,
    SwissBallDumbbellChestPress = 23,
    TripleStopBarbellBenchPress = 24,
    WideGripBarbellBenchPress = 25,
    AlternatingDumbbellChestPress = 26,
    Unknown,
}
impl BenchPressExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value . 0 { 0 => Ok ( BenchPressExerciseName :: AlternatingDumbbellChestPressOnSwissBall ) , 1 => Ok ( BenchPressExerciseName :: BarbellBenchPress ) , 2 => Ok ( BenchPressExerciseName :: BarbellBoardBenchPress ) , 3 => Ok ( BenchPressExerciseName :: BarbellFloorPress ) , 4 => Ok ( BenchPressExerciseName :: CloseGripBarbellBenchPress ) , 5 => Ok ( BenchPressExerciseName :: DeclineDumbbellBenchPress ) , 6 => Ok ( BenchPressExerciseName :: DumbbellBenchPress ) , 7 => Ok ( BenchPressExerciseName :: DumbbellFloorPress ) , 8 => Ok ( BenchPressExerciseName :: InclineBarbellBenchPress ) , 9 => Ok ( BenchPressExerciseName :: InclineDumbbellBenchPress ) , 10 => Ok ( BenchPressExerciseName :: InclineSmithMachineBenchPress ) , 11 => Ok ( BenchPressExerciseName :: IsometricBarbellBenchPress ) , 12 => Ok ( BenchPressExerciseName :: KettlebellChestPress ) , 13 => Ok ( BenchPressExerciseName :: NeutralGripDumbbellBenchPress ) , 14 => Ok ( BenchPressExerciseName :: NeutralGripDumbbellInclineBenchPress ) , 15 => Ok ( BenchPressExerciseName :: OneArmFloorPress ) , 16 => Ok ( BenchPressExerciseName :: WeightedOneArmFloorPress ) , 17 => Ok ( BenchPressExerciseName :: PartialLockout ) , 18 => Ok ( BenchPressExerciseName :: ReverseGripBarbellBenchPress ) , 19 => Ok ( BenchPressExerciseName :: ReverseGripInclineBenchPress ) , 20 => Ok ( BenchPressExerciseName :: SingleArmCableChestPress ) , 21 => Ok ( BenchPressExerciseName :: SingleArmDumbbellBenchPress ) , 22 => Ok ( BenchPressExerciseName :: SmithMachineBenchPress ) , 23 => Ok ( BenchPressExerciseName :: SwissBallDumbbellChestPress ) , 24 => Ok ( BenchPressExerciseName :: TripleStopBarbellBenchPress ) , 25 => Ok ( BenchPressExerciseName :: WideGripBarbellBenchPress ) , 26 => Ok ( BenchPressExerciseName :: AlternatingDumbbellChestPress ) , _ => Ok ( BenchPressExerciseName :: Unknown ) , }
    }
}
#[derive(Debug)]
pub enum CalfRaiseExerciseName {
    ThreeWayCalfRaise = 0,
    ThreeWayWeightedCalfRaise = 1,
    ThreeWaySingleLegCalfRaise = 2,
    ThreeWayWeightedSingleLegCalfRaise = 3,
    DonkeyCalfRaise = 4,
    WeightedDonkeyCalfRaise = 5,
    SeatedCalfRaise = 6,
    WeightedSeatedCalfRaise = 7,
    SeatedDumbbellToeRaise = 8,
    SingleLegBentKneeCalfRaise = 9,
    WeightedSingleLegBentKneeCalfRaise = 10,
    SingleLegDeclinePushUp = 11,
    SingleLegDonkeyCalfRaise = 12,
    WeightedSingleLegDonkeyCalfRaise = 13,
    SingleLegHipRaiseWithKneeHold = 14,
    SingleLegStandingCalfRaise = 15,
    SingleLegStandingDumbbellCalfRaise = 16,
    StandingBarbellCalfRaise = 17,
    StandingCalfRaise = 18,
    WeightedStandingCalfRaise = 19,
    StandingDumbbellCalfRaise = 20,
    Unknown,
}
impl CalfRaiseExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(CalfRaiseExerciseName::ThreeWayCalfRaise),
            1 => Ok(CalfRaiseExerciseName::ThreeWayWeightedCalfRaise),
            2 => Ok(CalfRaiseExerciseName::ThreeWaySingleLegCalfRaise),
            3 => Ok(CalfRaiseExerciseName::ThreeWayWeightedSingleLegCalfRaise),
            4 => Ok(CalfRaiseExerciseName::DonkeyCalfRaise),
            5 => Ok(CalfRaiseExerciseName::WeightedDonkeyCalfRaise),
            6 => Ok(CalfRaiseExerciseName::SeatedCalfRaise),
            7 => Ok(CalfRaiseExerciseName::WeightedSeatedCalfRaise),
            8 => Ok(CalfRaiseExerciseName::SeatedDumbbellToeRaise),
            9 => Ok(CalfRaiseExerciseName::SingleLegBentKneeCalfRaise),
            10 => Ok(CalfRaiseExerciseName::WeightedSingleLegBentKneeCalfRaise),
            11 => Ok(CalfRaiseExerciseName::SingleLegDeclinePushUp),
            12 => Ok(CalfRaiseExerciseName::SingleLegDonkeyCalfRaise),
            13 => Ok(CalfRaiseExerciseName::WeightedSingleLegDonkeyCalfRaise),
            14 => Ok(CalfRaiseExerciseName::SingleLegHipRaiseWithKneeHold),
            15 => Ok(CalfRaiseExerciseName::SingleLegStandingCalfRaise),
            16 => Ok(CalfRaiseExerciseName::SingleLegStandingDumbbellCalfRaise),
            17 => Ok(CalfRaiseExerciseName::StandingBarbellCalfRaise),
            18 => Ok(CalfRaiseExerciseName::StandingCalfRaise),
            19 => Ok(CalfRaiseExerciseName::WeightedStandingCalfRaise),
            20 => Ok(CalfRaiseExerciseName::StandingDumbbellCalfRaise),
            _ => Ok(CalfRaiseExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum CardioExerciseName {
    BobAndWeaveCircle = 0,
    WeightedBobAndWeaveCircle = 1,
    CardioCoreCrawl = 2,
    WeightedCardioCoreCrawl = 3,
    DoubleUnder = 4,
    WeightedDoubleUnder = 5,
    JumpRope = 6,
    WeightedJumpRope = 7,
    JumpRopeCrossover = 8,
    WeightedJumpRopeCrossover = 9,
    JumpRopeJog = 10,
    WeightedJumpRopeJog = 11,
    JumpingJacks = 12,
    WeightedJumpingJacks = 13,
    SkiMoguls = 14,
    WeightedSkiMoguls = 15,
    SplitJacks = 16,
    WeightedSplitJacks = 17,
    SquatJacks = 18,
    WeightedSquatJacks = 19,
    TripleUnder = 20,
    WeightedTripleUnder = 21,
    Unknown,
}
impl CardioExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(CardioExerciseName::BobAndWeaveCircle),
            1 => Ok(CardioExerciseName::WeightedBobAndWeaveCircle),
            2 => Ok(CardioExerciseName::CardioCoreCrawl),
            3 => Ok(CardioExerciseName::WeightedCardioCoreCrawl),
            4 => Ok(CardioExerciseName::DoubleUnder),
            5 => Ok(CardioExerciseName::WeightedDoubleUnder),
            6 => Ok(CardioExerciseName::JumpRope),
            7 => Ok(CardioExerciseName::WeightedJumpRope),
            8 => Ok(CardioExerciseName::JumpRopeCrossover),
            9 => Ok(CardioExerciseName::WeightedJumpRopeCrossover),
            10 => Ok(CardioExerciseName::JumpRopeJog),
            11 => Ok(CardioExerciseName::WeightedJumpRopeJog),
            12 => Ok(CardioExerciseName::JumpingJacks),
            13 => Ok(CardioExerciseName::WeightedJumpingJacks),
            14 => Ok(CardioExerciseName::SkiMoguls),
            15 => Ok(CardioExerciseName::WeightedSkiMoguls),
            16 => Ok(CardioExerciseName::SplitJacks),
            17 => Ok(CardioExerciseName::WeightedSplitJacks),
            18 => Ok(CardioExerciseName::SquatJacks),
            19 => Ok(CardioExerciseName::WeightedSquatJacks),
            20 => Ok(CardioExerciseName::TripleUnder),
            21 => Ok(CardioExerciseName::WeightedTripleUnder),
            _ => Ok(CardioExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum CarryExerciseName {
    BarHolds = 0,
    FarmersWalk = 1,
    FarmersWalkOnToes = 2,
    HexDumbbellHold = 3,
    OverheadCarry = 4,
    Unknown,
}
impl CarryExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(CarryExerciseName::BarHolds),
            1 => Ok(CarryExerciseName::FarmersWalk),
            2 => Ok(CarryExerciseName::FarmersWalkOnToes),
            3 => Ok(CarryExerciseName::HexDumbbellHold),
            4 => Ok(CarryExerciseName::OverheadCarry),
            _ => Ok(CarryExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ChopExerciseName {
    CablePullThrough = 0,
    CableRotationalLift = 1,
    CableWoodchop = 2,
    CrossChopToKnee = 3,
    WeightedCrossChopToKnee = 4,
    DumbbellChop = 5,
    HalfKneelingRotation = 6,
    WeightedHalfKneelingRotation = 7,
    HalfKneelingRotationalChop = 8,
    HalfKneelingRotationalReverseChop = 9,
    HalfKneelingStabilityChop = 10,
    HalfKneelingStabilityReverseChop = 11,
    KneelingRotationalChop = 12,
    KneelingRotationalReverseChop = 13,
    KneelingStabilityChop = 14,
    KneelingWoodchopper = 15,
    MedicineBallWoodChops = 16,
    PowerSquatChops = 17,
    WeightedPowerSquatChops = 18,
    StandingRotationalChop = 19,
    StandingSplitRotationalChop = 20,
    StandingSplitRotationalReverseChop = 21,
    StandingStabilityReverseChop = 22,
    Unknown,
}
impl ChopExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ChopExerciseName::CablePullThrough),
            1 => Ok(ChopExerciseName::CableRotationalLift),
            2 => Ok(ChopExerciseName::CableWoodchop),
            3 => Ok(ChopExerciseName::CrossChopToKnee),
            4 => Ok(ChopExerciseName::WeightedCrossChopToKnee),
            5 => Ok(ChopExerciseName::DumbbellChop),
            6 => Ok(ChopExerciseName::HalfKneelingRotation),
            7 => Ok(ChopExerciseName::WeightedHalfKneelingRotation),
            8 => Ok(ChopExerciseName::HalfKneelingRotationalChop),
            9 => Ok(ChopExerciseName::HalfKneelingRotationalReverseChop),
            10 => Ok(ChopExerciseName::HalfKneelingStabilityChop),
            11 => Ok(ChopExerciseName::HalfKneelingStabilityReverseChop),
            12 => Ok(ChopExerciseName::KneelingRotationalChop),
            13 => Ok(ChopExerciseName::KneelingRotationalReverseChop),
            14 => Ok(ChopExerciseName::KneelingStabilityChop),
            15 => Ok(ChopExerciseName::KneelingWoodchopper),
            16 => Ok(ChopExerciseName::MedicineBallWoodChops),
            17 => Ok(ChopExerciseName::PowerSquatChops),
            18 => Ok(ChopExerciseName::WeightedPowerSquatChops),
            19 => Ok(ChopExerciseName::StandingRotationalChop),
            20 => Ok(ChopExerciseName::StandingSplitRotationalChop),
            21 => Ok(ChopExerciseName::StandingSplitRotationalReverseChop),
            22 => Ok(ChopExerciseName::StandingStabilityReverseChop),
            _ => Ok(ChopExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum CoreExerciseName {
    AbsJabs = 0,
    WeightedAbsJabs = 1,
    AlternatingPlateReach = 2,
    BarbellRollout = 3,
    WeightedBarbellRollout = 4,
    BodyBarObliqueTwist = 5,
    CableCorePress = 6,
    CableSideBend = 7,
    SideBend = 8,
    WeightedSideBend = 9,
    CrescentCircle = 10,
    WeightedCrescentCircle = 11,
    CyclingRussianTwist = 12,
    WeightedCyclingRussianTwist = 13,
    ElevatedFeetRussianTwist = 14,
    WeightedElevatedFeetRussianTwist = 15,
    HalfTurkishGetUp = 16,
    KettlebellWindmill = 17,
    KneelingAbWheel = 18,
    WeightedKneelingAbWheel = 19,
    ModifiedFrontLever = 20,
    OpenKneeTucks = 21,
    WeightedOpenKneeTucks = 22,
    SideAbsLegLift = 23,
    WeightedSideAbsLegLift = 24,
    SwissBallJackknife = 25,
    WeightedSwissBallJackknife = 26,
    SwissBallPike = 27,
    WeightedSwissBallPike = 28,
    SwissBallRollout = 29,
    WeightedSwissBallRollout = 30,
    TriangleHipPress = 31,
    WeightedTriangleHipPress = 32,
    TrxSuspendedJackknife = 33,
    WeightedTrxSuspendedJackknife = 34,
    UBoat = 35,
    WeightedUBoat = 36,
    WindmillSwitches = 37,
    WeightedWindmillSwitches = 38,
    AlternatingSlideOut = 39,
    WeightedAlternatingSlideOut = 40,
    GhdBackExtensions = 41,
    WeightedGhdBackExtensions = 42,
    OverheadWalk = 43,
    Inchworm = 44,
    WeightedModifiedFrontLever = 45,
    Unknown,
}
impl CoreExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(CoreExerciseName::AbsJabs),
            1 => Ok(CoreExerciseName::WeightedAbsJabs),
            2 => Ok(CoreExerciseName::AlternatingPlateReach),
            3 => Ok(CoreExerciseName::BarbellRollout),
            4 => Ok(CoreExerciseName::WeightedBarbellRollout),
            5 => Ok(CoreExerciseName::BodyBarObliqueTwist),
            6 => Ok(CoreExerciseName::CableCorePress),
            7 => Ok(CoreExerciseName::CableSideBend),
            8 => Ok(CoreExerciseName::SideBend),
            9 => Ok(CoreExerciseName::WeightedSideBend),
            10 => Ok(CoreExerciseName::CrescentCircle),
            11 => Ok(CoreExerciseName::WeightedCrescentCircle),
            12 => Ok(CoreExerciseName::CyclingRussianTwist),
            13 => Ok(CoreExerciseName::WeightedCyclingRussianTwist),
            14 => Ok(CoreExerciseName::ElevatedFeetRussianTwist),
            15 => Ok(CoreExerciseName::WeightedElevatedFeetRussianTwist),
            16 => Ok(CoreExerciseName::HalfTurkishGetUp),
            17 => Ok(CoreExerciseName::KettlebellWindmill),
            18 => Ok(CoreExerciseName::KneelingAbWheel),
            19 => Ok(CoreExerciseName::WeightedKneelingAbWheel),
            20 => Ok(CoreExerciseName::ModifiedFrontLever),
            21 => Ok(CoreExerciseName::OpenKneeTucks),
            22 => Ok(CoreExerciseName::WeightedOpenKneeTucks),
            23 => Ok(CoreExerciseName::SideAbsLegLift),
            24 => Ok(CoreExerciseName::WeightedSideAbsLegLift),
            25 => Ok(CoreExerciseName::SwissBallJackknife),
            26 => Ok(CoreExerciseName::WeightedSwissBallJackknife),
            27 => Ok(CoreExerciseName::SwissBallPike),
            28 => Ok(CoreExerciseName::WeightedSwissBallPike),
            29 => Ok(CoreExerciseName::SwissBallRollout),
            30 => Ok(CoreExerciseName::WeightedSwissBallRollout),
            31 => Ok(CoreExerciseName::TriangleHipPress),
            32 => Ok(CoreExerciseName::WeightedTriangleHipPress),
            33 => Ok(CoreExerciseName::TrxSuspendedJackknife),
            34 => Ok(CoreExerciseName::WeightedTrxSuspendedJackknife),
            35 => Ok(CoreExerciseName::UBoat),
            36 => Ok(CoreExerciseName::WeightedUBoat),
            37 => Ok(CoreExerciseName::WindmillSwitches),
            38 => Ok(CoreExerciseName::WeightedWindmillSwitches),
            39 => Ok(CoreExerciseName::AlternatingSlideOut),
            40 => Ok(CoreExerciseName::WeightedAlternatingSlideOut),
            41 => Ok(CoreExerciseName::GhdBackExtensions),
            42 => Ok(CoreExerciseName::WeightedGhdBackExtensions),
            43 => Ok(CoreExerciseName::OverheadWalk),
            44 => Ok(CoreExerciseName::Inchworm),
            45 => Ok(CoreExerciseName::WeightedModifiedFrontLever),
            _ => Ok(CoreExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum CrunchExerciseName {
    BicycleCrunch = 0,
    CableCrunch = 1,
    CircularArmCrunch = 2,
    CrossedArmsCrunch = 3,
    WeightedCrossedArmsCrunch = 4,
    CrossLegReverseCrunch = 5,
    WeightedCrossLegReverseCrunch = 6,
    CrunchChop = 7,
    WeightedCrunchChop = 8,
    DoubleCrunch = 9,
    WeightedDoubleCrunch = 10,
    ElbowToKneeCrunch = 11,
    WeightedElbowToKneeCrunch = 12,
    FlutterKicks = 13,
    WeightedFlutterKicks = 14,
    FoamRollerReverseCrunchOnBench = 15,
    WeightedFoamRollerReverseCrunchOnBench = 16,
    FoamRollerReverseCrunchWithDumbbell = 17,
    FoamRollerReverseCrunchWithMedicineBall = 18,
    FrogPress = 19,
    HangingKneeRaiseObliqueCrunch = 20,
    WeightedHangingKneeRaiseObliqueCrunch = 21,
    HipCrossover = 22,
    WeightedHipCrossover = 23,
    HollowRock = 24,
    WeightedHollowRock = 25,
    InclineReverseCrunch = 26,
    WeightedInclineReverseCrunch = 27,
    KneelingCableCrunch = 28,
    KneelingCrossCrunch = 29,
    WeightedKneelingCrossCrunch = 30,
    KneelingObliqueCableCrunch = 31,
    KneesToElbow = 32,
    LegExtensions = 33,
    WeightedLegExtensions = 34,
    LegLevers = 35,
    McgillCurlUp = 36,
    WeightedMcgillCurlUp = 37,
    ModifiedPilatesRollUpWithBall = 38,
    WeightedModifiedPilatesRollUpWithBall = 39,
    PilatesCrunch = 40,
    WeightedPilatesCrunch = 41,
    PilatesRollUpWithBall = 42,
    WeightedPilatesRollUpWithBall = 43,
    RaisedLegsCrunch = 44,
    WeightedRaisedLegsCrunch = 45,
    ReverseCrunch = 46,
    WeightedReverseCrunch = 47,
    ReverseCrunchOnABench = 48,
    WeightedReverseCrunchOnABench = 49,
    ReverseCurlAndLift = 50,
    WeightedReverseCurlAndLift = 51,
    RotationalLift = 52,
    WeightedRotationalLift = 53,
    SeatedAlternatingReverseCrunch = 54,
    WeightedSeatedAlternatingReverseCrunch = 55,
    SeatedLegU = 56,
    WeightedSeatedLegU = 57,
    SideToSideCrunchAndWeave = 58,
    WeightedSideToSideCrunchAndWeave = 59,
    SingleLegReverseCrunch = 60,
    WeightedSingleLegReverseCrunch = 61,
    SkaterCrunchCross = 62,
    WeightedSkaterCrunchCross = 63,
    StandingCableCrunch = 64,
    StandingSideCrunch = 65,
    StepClimb = 66,
    WeightedStepClimb = 67,
    SwissBallCrunch = 68,
    SwissBallReverseCrunch = 69,
    WeightedSwissBallReverseCrunch = 70,
    SwissBallRussianTwist = 71,
    WeightedSwissBallRussianTwist = 72,
    SwissBallSideCrunch = 73,
    WeightedSwissBallSideCrunch = 74,
    ThoracicCrunchesOnFoamRoller = 75,
    WeightedThoracicCrunchesOnFoamRoller = 76,
    TricepsCrunch = 77,
    WeightedBicycleCrunch = 78,
    WeightedCrunch = 79,
    WeightedSwissBallCrunch = 80,
    ToesToBar = 81,
    WeightedToesToBar = 82,
    Crunch = 83,
    Unknown,
}
impl CrunchExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(CrunchExerciseName::BicycleCrunch),
            1 => Ok(CrunchExerciseName::CableCrunch),
            2 => Ok(CrunchExerciseName::CircularArmCrunch),
            3 => Ok(CrunchExerciseName::CrossedArmsCrunch),
            4 => Ok(CrunchExerciseName::WeightedCrossedArmsCrunch),
            5 => Ok(CrunchExerciseName::CrossLegReverseCrunch),
            6 => Ok(CrunchExerciseName::WeightedCrossLegReverseCrunch),
            7 => Ok(CrunchExerciseName::CrunchChop),
            8 => Ok(CrunchExerciseName::WeightedCrunchChop),
            9 => Ok(CrunchExerciseName::DoubleCrunch),
            10 => Ok(CrunchExerciseName::WeightedDoubleCrunch),
            11 => Ok(CrunchExerciseName::ElbowToKneeCrunch),
            12 => Ok(CrunchExerciseName::WeightedElbowToKneeCrunch),
            13 => Ok(CrunchExerciseName::FlutterKicks),
            14 => Ok(CrunchExerciseName::WeightedFlutterKicks),
            15 => Ok(CrunchExerciseName::FoamRollerReverseCrunchOnBench),
            16 => {
                Ok(CrunchExerciseName::WeightedFoamRollerReverseCrunchOnBench)
            },
            17 => Ok(CrunchExerciseName::FoamRollerReverseCrunchWithDumbbell),
            18 => {
                Ok(CrunchExerciseName::FoamRollerReverseCrunchWithMedicineBall)
            },
            19 => Ok(CrunchExerciseName::FrogPress),
            20 => Ok(CrunchExerciseName::HangingKneeRaiseObliqueCrunch),
            21 => Ok(CrunchExerciseName::WeightedHangingKneeRaiseObliqueCrunch),
            22 => Ok(CrunchExerciseName::HipCrossover),
            23 => Ok(CrunchExerciseName::WeightedHipCrossover),
            24 => Ok(CrunchExerciseName::HollowRock),
            25 => Ok(CrunchExerciseName::WeightedHollowRock),
            26 => Ok(CrunchExerciseName::InclineReverseCrunch),
            27 => Ok(CrunchExerciseName::WeightedInclineReverseCrunch),
            28 => Ok(CrunchExerciseName::KneelingCableCrunch),
            29 => Ok(CrunchExerciseName::KneelingCrossCrunch),
            30 => Ok(CrunchExerciseName::WeightedKneelingCrossCrunch),
            31 => Ok(CrunchExerciseName::KneelingObliqueCableCrunch),
            32 => Ok(CrunchExerciseName::KneesToElbow),
            33 => Ok(CrunchExerciseName::LegExtensions),
            34 => Ok(CrunchExerciseName::WeightedLegExtensions),
            35 => Ok(CrunchExerciseName::LegLevers),
            36 => Ok(CrunchExerciseName::McgillCurlUp),
            37 => Ok(CrunchExerciseName::WeightedMcgillCurlUp),
            38 => Ok(CrunchExerciseName::ModifiedPilatesRollUpWithBall),
            39 => Ok(CrunchExerciseName::WeightedModifiedPilatesRollUpWithBall),
            40 => Ok(CrunchExerciseName::PilatesCrunch),
            41 => Ok(CrunchExerciseName::WeightedPilatesCrunch),
            42 => Ok(CrunchExerciseName::PilatesRollUpWithBall),
            43 => Ok(CrunchExerciseName::WeightedPilatesRollUpWithBall),
            44 => Ok(CrunchExerciseName::RaisedLegsCrunch),
            45 => Ok(CrunchExerciseName::WeightedRaisedLegsCrunch),
            46 => Ok(CrunchExerciseName::ReverseCrunch),
            47 => Ok(CrunchExerciseName::WeightedReverseCrunch),
            48 => Ok(CrunchExerciseName::ReverseCrunchOnABench),
            49 => Ok(CrunchExerciseName::WeightedReverseCrunchOnABench),
            50 => Ok(CrunchExerciseName::ReverseCurlAndLift),
            51 => Ok(CrunchExerciseName::WeightedReverseCurlAndLift),
            52 => Ok(CrunchExerciseName::RotationalLift),
            53 => Ok(CrunchExerciseName::WeightedRotationalLift),
            54 => Ok(CrunchExerciseName::SeatedAlternatingReverseCrunch),
            55 => {
                Ok(CrunchExerciseName::WeightedSeatedAlternatingReverseCrunch)
            },
            56 => Ok(CrunchExerciseName::SeatedLegU),
            57 => Ok(CrunchExerciseName::WeightedSeatedLegU),
            58 => Ok(CrunchExerciseName::SideToSideCrunchAndWeave),
            59 => Ok(CrunchExerciseName::WeightedSideToSideCrunchAndWeave),
            60 => Ok(CrunchExerciseName::SingleLegReverseCrunch),
            61 => Ok(CrunchExerciseName::WeightedSingleLegReverseCrunch),
            62 => Ok(CrunchExerciseName::SkaterCrunchCross),
            63 => Ok(CrunchExerciseName::WeightedSkaterCrunchCross),
            64 => Ok(CrunchExerciseName::StandingCableCrunch),
            65 => Ok(CrunchExerciseName::StandingSideCrunch),
            66 => Ok(CrunchExerciseName::StepClimb),
            67 => Ok(CrunchExerciseName::WeightedStepClimb),
            68 => Ok(CrunchExerciseName::SwissBallCrunch),
            69 => Ok(CrunchExerciseName::SwissBallReverseCrunch),
            70 => Ok(CrunchExerciseName::WeightedSwissBallReverseCrunch),
            71 => Ok(CrunchExerciseName::SwissBallRussianTwist),
            72 => Ok(CrunchExerciseName::WeightedSwissBallRussianTwist),
            73 => Ok(CrunchExerciseName::SwissBallSideCrunch),
            74 => Ok(CrunchExerciseName::WeightedSwissBallSideCrunch),
            75 => Ok(CrunchExerciseName::ThoracicCrunchesOnFoamRoller),
            76 => Ok(CrunchExerciseName::WeightedThoracicCrunchesOnFoamRoller),
            77 => Ok(CrunchExerciseName::TricepsCrunch),
            78 => Ok(CrunchExerciseName::WeightedBicycleCrunch),
            79 => Ok(CrunchExerciseName::WeightedCrunch),
            80 => Ok(CrunchExerciseName::WeightedSwissBallCrunch),
            81 => Ok(CrunchExerciseName::ToesToBar),
            82 => Ok(CrunchExerciseName::WeightedToesToBar),
            83 => Ok(CrunchExerciseName::Crunch),
            _ => Ok(CrunchExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum CurlExerciseName {
    AlternatingDumbbellBicepsCurl = 0,
    AlternatingDumbbellBicepsCurlOnSwissBall = 1,
    AlternatingInclineDumbbellBicepsCurl = 2,
    BarbellBicepsCurl = 3,
    BarbellReverseWristCurl = 4,
    BarbellWristCurl = 5,
    BehindTheBackBarbellReverseWristCurl = 6,
    BehindTheBackOneArmCableCurl = 7,
    CableBicepsCurl = 8,
    CableHammerCurl = 9,
    CheatingBarbellBicepsCurl = 10,
    CloseGripEzBarBicepsCurl = 11,
    CrossBodyDumbbellHammerCurl = 12,
    DeadHangBicepsCurl = 13,
    DeclineHammerCurl = 14,
    DumbbellBicepsCurlWithStaticHold = 15,
    DumbbellHammerCurl = 16,
    DumbbellReverseWristCurl = 17,
    DumbbellWristCurl = 18,
    EzBarPreacherCurl = 19,
    ForwardBendBicepsCurl = 20,
    HammerCurlToPress = 21,
    InclineDumbbellBicepsCurl = 22,
    InclineOffsetThumbDumbbellCurl = 23,
    KettlebellBicepsCurl = 24,
    LyingConcentrationCableCurl = 25,
    OneArmPreacherCurl = 26,
    PlatePinchCurl = 27,
    PreacherCurlWithCable = 28,
    ReverseEzBarCurl = 29,
    ReverseGripWristCurl = 30,
    ReverseGripBarbellBicepsCurl = 31,
    SeatedAlternatingDumbbellBicepsCurl = 32,
    SeatedDumbbellBicepsCurl = 33,
    SeatedReverseDumbbellCurl = 34,
    SplitStanceOffsetPinkyDumbbellCurl = 35,
    StandingAlternatingDumbbellCurls = 36,
    StandingDumbbellBicepsCurl = 37,
    StandingEzBarBicepsCurl = 38,
    StaticCurl = 39,
    SwissBallDumbbellOverheadTricepsExtension = 40,
    SwissBallEzBarPreacherCurl = 41,
    TwistingStandingDumbbellBicepsCurl = 42,
    WideGripEzBarBicepsCurl = 43,
    Unknown,
}
impl CurlExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(CurlExerciseName::AlternatingDumbbellBicepsCurl),
            1 => Ok(CurlExerciseName::AlternatingDumbbellBicepsCurlOnSwissBall),
            2 => Ok(CurlExerciseName::AlternatingInclineDumbbellBicepsCurl),
            3 => Ok(CurlExerciseName::BarbellBicepsCurl),
            4 => Ok(CurlExerciseName::BarbellReverseWristCurl),
            5 => Ok(CurlExerciseName::BarbellWristCurl),
            6 => Ok(CurlExerciseName::BehindTheBackBarbellReverseWristCurl),
            7 => Ok(CurlExerciseName::BehindTheBackOneArmCableCurl),
            8 => Ok(CurlExerciseName::CableBicepsCurl),
            9 => Ok(CurlExerciseName::CableHammerCurl),
            10 => Ok(CurlExerciseName::CheatingBarbellBicepsCurl),
            11 => Ok(CurlExerciseName::CloseGripEzBarBicepsCurl),
            12 => Ok(CurlExerciseName::CrossBodyDumbbellHammerCurl),
            13 => Ok(CurlExerciseName::DeadHangBicepsCurl),
            14 => Ok(CurlExerciseName::DeclineHammerCurl),
            15 => Ok(CurlExerciseName::DumbbellBicepsCurlWithStaticHold),
            16 => Ok(CurlExerciseName::DumbbellHammerCurl),
            17 => Ok(CurlExerciseName::DumbbellReverseWristCurl),
            18 => Ok(CurlExerciseName::DumbbellWristCurl),
            19 => Ok(CurlExerciseName::EzBarPreacherCurl),
            20 => Ok(CurlExerciseName::ForwardBendBicepsCurl),
            21 => Ok(CurlExerciseName::HammerCurlToPress),
            22 => Ok(CurlExerciseName::InclineDumbbellBicepsCurl),
            23 => Ok(CurlExerciseName::InclineOffsetThumbDumbbellCurl),
            24 => Ok(CurlExerciseName::KettlebellBicepsCurl),
            25 => Ok(CurlExerciseName::LyingConcentrationCableCurl),
            26 => Ok(CurlExerciseName::OneArmPreacherCurl),
            27 => Ok(CurlExerciseName::PlatePinchCurl),
            28 => Ok(CurlExerciseName::PreacherCurlWithCable),
            29 => Ok(CurlExerciseName::ReverseEzBarCurl),
            30 => Ok(CurlExerciseName::ReverseGripWristCurl),
            31 => Ok(CurlExerciseName::ReverseGripBarbellBicepsCurl),
            32 => Ok(CurlExerciseName::SeatedAlternatingDumbbellBicepsCurl),
            33 => Ok(CurlExerciseName::SeatedDumbbellBicepsCurl),
            34 => Ok(CurlExerciseName::SeatedReverseDumbbellCurl),
            35 => Ok(CurlExerciseName::SplitStanceOffsetPinkyDumbbellCurl),
            36 => Ok(CurlExerciseName::StandingAlternatingDumbbellCurls),
            37 => Ok(CurlExerciseName::StandingDumbbellBicepsCurl),
            38 => Ok(CurlExerciseName::StandingEzBarBicepsCurl),
            39 => Ok(CurlExerciseName::StaticCurl),
            40 => {
                Ok(CurlExerciseName::SwissBallDumbbellOverheadTricepsExtension)
            },
            41 => Ok(CurlExerciseName::SwissBallEzBarPreacherCurl),
            42 => Ok(CurlExerciseName::TwistingStandingDumbbellBicepsCurl),
            43 => Ok(CurlExerciseName::WideGripEzBarBicepsCurl),
            _ => Ok(CurlExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DeadliftExerciseName {
    BarbellDeadlift = 0,
    BarbellStraightLegDeadlift = 1,
    DumbbellDeadlift = 2,
    DumbbellSingleLegDeadliftToRow = 3,
    DumbbellStraightLegDeadlift = 4,
    KettlebellFloorToShelf = 5,
    OneArmOneLegDeadlift = 6,
    RackPull = 7,
    RotationalDumbbellStraightLegDeadlift = 8,
    SingleArmDeadlift = 9,
    SingleLegBarbellDeadlift = 10,
    SingleLegBarbellStraightLegDeadlift = 11,
    SingleLegDeadliftWithBarbell = 12,
    SingleLegRdlCircuit = 13,
    SingleLegRomanianDeadliftWithDumbbell = 14,
    SumoDeadlift = 15,
    SumoDeadliftHighPull = 16,
    TrapBarDeadlift = 17,
    WideGripBarbellDeadlift = 18,
    Unknown,
}
impl DeadliftExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DeadliftExerciseName::BarbellDeadlift),
            1 => Ok(DeadliftExerciseName::BarbellStraightLegDeadlift),
            2 => Ok(DeadliftExerciseName::DumbbellDeadlift),
            3 => Ok(DeadliftExerciseName::DumbbellSingleLegDeadliftToRow),
            4 => Ok(DeadliftExerciseName::DumbbellStraightLegDeadlift),
            5 => Ok(DeadliftExerciseName::KettlebellFloorToShelf),
            6 => Ok(DeadliftExerciseName::OneArmOneLegDeadlift),
            7 => Ok(DeadliftExerciseName::RackPull),
            8 => {
                Ok(DeadliftExerciseName::RotationalDumbbellStraightLegDeadlift)
            },
            9 => Ok(DeadliftExerciseName::SingleArmDeadlift),
            10 => Ok(DeadliftExerciseName::SingleLegBarbellDeadlift),
            11 => Ok(DeadliftExerciseName::SingleLegBarbellStraightLegDeadlift),
            12 => Ok(DeadliftExerciseName::SingleLegDeadliftWithBarbell),
            13 => Ok(DeadliftExerciseName::SingleLegRdlCircuit),
            14 => {
                Ok(DeadliftExerciseName::SingleLegRomanianDeadliftWithDumbbell)
            },
            15 => Ok(DeadliftExerciseName::SumoDeadlift),
            16 => Ok(DeadliftExerciseName::SumoDeadliftHighPull),
            17 => Ok(DeadliftExerciseName::TrapBarDeadlift),
            18 => Ok(DeadliftExerciseName::WideGripBarbellDeadlift),
            _ => Ok(DeadliftExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum FlyeExerciseName {
    CableCrossover = 0,
    DeclineDumbbellFlye = 1,
    DumbbellFlye = 2,
    InclineDumbbellFlye = 3,
    KettlebellFlye = 4,
    KneelingRearFlye = 5,
    SingleArmStandingCableReverseFlye = 6,
    SwissBallDumbbellFlye = 7,
    Unknown,
}
impl FlyeExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(FlyeExerciseName::CableCrossover),
            1 => Ok(FlyeExerciseName::DeclineDumbbellFlye),
            2 => Ok(FlyeExerciseName::DumbbellFlye),
            3 => Ok(FlyeExerciseName::InclineDumbbellFlye),
            4 => Ok(FlyeExerciseName::KettlebellFlye),
            5 => Ok(FlyeExerciseName::KneelingRearFlye),
            6 => Ok(FlyeExerciseName::SingleArmStandingCableReverseFlye),
            7 => Ok(FlyeExerciseName::SwissBallDumbbellFlye),
            _ => Ok(FlyeExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum HipRaiseExerciseName {
    BarbellHipThrustOnFloor = 0,
    BarbellHipThrustWithBench = 1,
    BentKneeSwissBallReverseHipRaise = 2,
    WeightedBentKneeSwissBallReverseHipRaise = 3,
    BridgeWithLegExtension = 4,
    WeightedBridgeWithLegExtension = 5,
    ClamBridge = 6,
    FrontKickTabletop = 7,
    WeightedFrontKickTabletop = 8,
    HipExtensionAndCross = 9,
    WeightedHipExtensionAndCross = 10,
    HipRaise = 11,
    WeightedHipRaise = 12,
    HipRaiseWithFeetOnSwissBall = 13,
    WeightedHipRaiseWithFeetOnSwissBall = 14,
    HipRaiseWithHeadOnBosuBall = 15,
    WeightedHipRaiseWithHeadOnBosuBall = 16,
    HipRaiseWithHeadOnSwissBall = 17,
    WeightedHipRaiseWithHeadOnSwissBall = 18,
    HipRaiseWithKneeSqueeze = 19,
    WeightedHipRaiseWithKneeSqueeze = 20,
    InclineRearLegExtension = 21,
    WeightedInclineRearLegExtension = 22,
    KettlebellSwing = 23,
    MarchingHipRaise = 24,
    WeightedMarchingHipRaise = 25,
    MarchingHipRaiseWithFeetOnASwissBall = 26,
    WeightedMarchingHipRaiseWithFeetOnASwissBall = 27,
    ReverseHipRaise = 28,
    WeightedReverseHipRaise = 29,
    SingleLegHipRaise = 30,
    WeightedSingleLegHipRaise = 31,
    SingleLegHipRaiseWithFootOnBench = 32,
    WeightedSingleLegHipRaiseWithFootOnBench = 33,
    SingleLegHipRaiseWithFootOnBosuBall = 34,
    WeightedSingleLegHipRaiseWithFootOnBosuBall = 35,
    SingleLegHipRaiseWithFootOnFoamRoller = 36,
    WeightedSingleLegHipRaiseWithFootOnFoamRoller = 37,
    SingleLegHipRaiseWithFootOnMedicineBall = 38,
    WeightedSingleLegHipRaiseWithFootOnMedicineBall = 39,
    SingleLegHipRaiseWithHeadOnBosuBall = 40,
    WeightedSingleLegHipRaiseWithHeadOnBosuBall = 41,
    WeightedClamBridge = 42,
    Unknown,
}
impl HipRaiseExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value . 0 { 0 => Ok ( HipRaiseExerciseName :: BarbellHipThrustOnFloor ) , 1 => Ok ( HipRaiseExerciseName :: BarbellHipThrustWithBench ) , 2 => Ok ( HipRaiseExerciseName :: BentKneeSwissBallReverseHipRaise ) , 3 => Ok ( HipRaiseExerciseName :: WeightedBentKneeSwissBallReverseHipRaise ) , 4 => Ok ( HipRaiseExerciseName :: BridgeWithLegExtension ) , 5 => Ok ( HipRaiseExerciseName :: WeightedBridgeWithLegExtension ) , 6 => Ok ( HipRaiseExerciseName :: ClamBridge ) , 7 => Ok ( HipRaiseExerciseName :: FrontKickTabletop ) , 8 => Ok ( HipRaiseExerciseName :: WeightedFrontKickTabletop ) , 9 => Ok ( HipRaiseExerciseName :: HipExtensionAndCross ) , 10 => Ok ( HipRaiseExerciseName :: WeightedHipExtensionAndCross ) , 11 => Ok ( HipRaiseExerciseName :: HipRaise ) , 12 => Ok ( HipRaiseExerciseName :: WeightedHipRaise ) , 13 => Ok ( HipRaiseExerciseName :: HipRaiseWithFeetOnSwissBall ) , 14 => Ok ( HipRaiseExerciseName :: WeightedHipRaiseWithFeetOnSwissBall ) , 15 => Ok ( HipRaiseExerciseName :: HipRaiseWithHeadOnBosuBall ) , 16 => Ok ( HipRaiseExerciseName :: WeightedHipRaiseWithHeadOnBosuBall ) , 17 => Ok ( HipRaiseExerciseName :: HipRaiseWithHeadOnSwissBall ) , 18 => Ok ( HipRaiseExerciseName :: WeightedHipRaiseWithHeadOnSwissBall ) , 19 => Ok ( HipRaiseExerciseName :: HipRaiseWithKneeSqueeze ) , 20 => Ok ( HipRaiseExerciseName :: WeightedHipRaiseWithKneeSqueeze ) , 21 => Ok ( HipRaiseExerciseName :: InclineRearLegExtension ) , 22 => Ok ( HipRaiseExerciseName :: WeightedInclineRearLegExtension ) , 23 => Ok ( HipRaiseExerciseName :: KettlebellSwing ) , 24 => Ok ( HipRaiseExerciseName :: MarchingHipRaise ) , 25 => Ok ( HipRaiseExerciseName :: WeightedMarchingHipRaise ) , 26 => Ok ( HipRaiseExerciseName :: MarchingHipRaiseWithFeetOnASwissBall ) , 27 => Ok ( HipRaiseExerciseName :: WeightedMarchingHipRaiseWithFeetOnASwissBall ) , 28 => Ok ( HipRaiseExerciseName :: ReverseHipRaise ) , 29 => Ok ( HipRaiseExerciseName :: WeightedReverseHipRaise ) , 30 => Ok ( HipRaiseExerciseName :: SingleLegHipRaise ) , 31 => Ok ( HipRaiseExerciseName :: WeightedSingleLegHipRaise ) , 32 => Ok ( HipRaiseExerciseName :: SingleLegHipRaiseWithFootOnBench ) , 33 => Ok ( HipRaiseExerciseName :: WeightedSingleLegHipRaiseWithFootOnBench ) , 34 => Ok ( HipRaiseExerciseName :: SingleLegHipRaiseWithFootOnBosuBall ) , 35 => Ok ( HipRaiseExerciseName :: WeightedSingleLegHipRaiseWithFootOnBosuBall ) , 36 => Ok ( HipRaiseExerciseName :: SingleLegHipRaiseWithFootOnFoamRoller ) , 37 => Ok ( HipRaiseExerciseName :: WeightedSingleLegHipRaiseWithFootOnFoamRoller ) , 38 => Ok ( HipRaiseExerciseName :: SingleLegHipRaiseWithFootOnMedicineBall ) , 39 => Ok ( HipRaiseExerciseName :: WeightedSingleLegHipRaiseWithFootOnMedicineBall ) , 40 => Ok ( HipRaiseExerciseName :: SingleLegHipRaiseWithHeadOnBosuBall ) , 41 => Ok ( HipRaiseExerciseName :: WeightedSingleLegHipRaiseWithHeadOnBosuBall ) , 42 => Ok ( HipRaiseExerciseName :: WeightedClamBridge ) , _ => Ok ( HipRaiseExerciseName :: Unknown ) , }
    }
}
#[derive(Debug)]
pub enum HipStabilityExerciseName {
    BandSideLyingLegRaise = 0,
    DeadBug = 1,
    WeightedDeadBug = 2,
    ExternalHipRaise = 3,
    WeightedExternalHipRaise = 4,
    FireHydrantKicks = 5,
    WeightedFireHydrantKicks = 6,
    HipCircles = 7,
    WeightedHipCircles = 8,
    InnerThighLift = 9,
    WeightedInnerThighLift = 10,
    LateralWalksWithBandAtAnkles = 11,
    PretzelSideKick = 12,
    WeightedPretzelSideKick = 13,
    ProneHipInternalRotation = 14,
    WeightedProneHipInternalRotation = 15,
    Quadruped = 16,
    QuadrupedHipExtension = 17,
    WeightedQuadrupedHipExtension = 18,
    QuadrupedWithLegLift = 19,
    WeightedQuadrupedWithLegLift = 20,
    SideLyingLegRaise = 21,
    WeightedSideLyingLegRaise = 22,
    SlidingHipAdduction = 23,
    WeightedSlidingHipAdduction = 24,
    StandingAdduction = 25,
    WeightedStandingAdduction = 26,
    StandingCableHipAbduction = 27,
    StandingHipAbduction = 28,
    WeightedStandingHipAbduction = 29,
    StandingRearLegRaise = 30,
    WeightedStandingRearLegRaise = 31,
    SupineHipInternalRotation = 32,
    WeightedSupineHipInternalRotation = 33,
    Unknown,
}
impl HipStabilityExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(HipStabilityExerciseName::BandSideLyingLegRaise),
            1 => Ok(HipStabilityExerciseName::DeadBug),
            2 => Ok(HipStabilityExerciseName::WeightedDeadBug),
            3 => Ok(HipStabilityExerciseName::ExternalHipRaise),
            4 => Ok(HipStabilityExerciseName::WeightedExternalHipRaise),
            5 => Ok(HipStabilityExerciseName::FireHydrantKicks),
            6 => Ok(HipStabilityExerciseName::WeightedFireHydrantKicks),
            7 => Ok(HipStabilityExerciseName::HipCircles),
            8 => Ok(HipStabilityExerciseName::WeightedHipCircles),
            9 => Ok(HipStabilityExerciseName::InnerThighLift),
            10 => Ok(HipStabilityExerciseName::WeightedInnerThighLift),
            11 => Ok(HipStabilityExerciseName::LateralWalksWithBandAtAnkles),
            12 => Ok(HipStabilityExerciseName::PretzelSideKick),
            13 => Ok(HipStabilityExerciseName::WeightedPretzelSideKick),
            14 => Ok(HipStabilityExerciseName::ProneHipInternalRotation),
            15 => {
                Ok(HipStabilityExerciseName::WeightedProneHipInternalRotation)
            },
            16 => Ok(HipStabilityExerciseName::Quadruped),
            17 => Ok(HipStabilityExerciseName::QuadrupedHipExtension),
            18 => Ok(HipStabilityExerciseName::WeightedQuadrupedHipExtension),
            19 => Ok(HipStabilityExerciseName::QuadrupedWithLegLift),
            20 => Ok(HipStabilityExerciseName::WeightedQuadrupedWithLegLift),
            21 => Ok(HipStabilityExerciseName::SideLyingLegRaise),
            22 => Ok(HipStabilityExerciseName::WeightedSideLyingLegRaise),
            23 => Ok(HipStabilityExerciseName::SlidingHipAdduction),
            24 => Ok(HipStabilityExerciseName::WeightedSlidingHipAdduction),
            25 => Ok(HipStabilityExerciseName::StandingAdduction),
            26 => Ok(HipStabilityExerciseName::WeightedStandingAdduction),
            27 => Ok(HipStabilityExerciseName::StandingCableHipAbduction),
            28 => Ok(HipStabilityExerciseName::StandingHipAbduction),
            29 => Ok(HipStabilityExerciseName::WeightedStandingHipAbduction),
            30 => Ok(HipStabilityExerciseName::StandingRearLegRaise),
            31 => Ok(HipStabilityExerciseName::WeightedStandingRearLegRaise),
            32 => Ok(HipStabilityExerciseName::SupineHipInternalRotation),
            33 => {
                Ok(HipStabilityExerciseName::WeightedSupineHipInternalRotation)
            },
            _ => Ok(HipStabilityExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum HipSwingExerciseName {
    SingleArmKettlebellSwing = 0,
    SingleArmDumbbellSwing = 1,
    StepOutSwing = 2,
    Unknown,
}
impl HipSwingExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(HipSwingExerciseName::SingleArmKettlebellSwing),
            1 => Ok(HipSwingExerciseName::SingleArmDumbbellSwing),
            2 => Ok(HipSwingExerciseName::StepOutSwing),
            _ => Ok(HipSwingExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum HyperextensionExerciseName {
    BackExtensionWithOppositeArmAndLegReach = 0,
    WeightedBackExtensionWithOppositeArmAndLegReach = 1,
    BaseRotations = 2,
    WeightedBaseRotations = 3,
    BentKneeReverseHyperextension = 4,
    WeightedBentKneeReverseHyperextension = 5,
    HollowHoldAndRoll = 6,
    WeightedHollowHoldAndRoll = 7,
    Kicks = 8,
    WeightedKicks = 9,
    KneeRaises = 10,
    WeightedKneeRaises = 11,
    KneelingSuperman = 12,
    WeightedKneelingSuperman = 13,
    LatPullDownWithRow = 14,
    MedicineBallDeadliftToReach = 15,
    OneArmOneLegRow = 16,
    OneArmRowWithBand = 17,
    OverheadLungeWithMedicineBall = 18,
    PlankKneeTucks = 19,
    WeightedPlankKneeTucks = 20,
    SideStep = 21,
    WeightedSideStep = 22,
    SingleLegBackExtension = 23,
    WeightedSingleLegBackExtension = 24,
    SpineExtension = 25,
    WeightedSpineExtension = 26,
    StaticBackExtension = 27,
    WeightedStaticBackExtension = 28,
    SupermanFromFloor = 29,
    WeightedSupermanFromFloor = 30,
    SwissBallBackExtension = 31,
    WeightedSwissBallBackExtension = 32,
    SwissBallHyperextension = 33,
    WeightedSwissBallHyperextension = 34,
    SwissBallOppositeArmAndLegLift = 35,
    WeightedSwissBallOppositeArmAndLegLift = 36,
    Unknown,
}
impl HyperextensionExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value . 0 { 0 => Ok ( HyperextensionExerciseName :: BackExtensionWithOppositeArmAndLegReach ) , 1 => Ok ( HyperextensionExerciseName :: WeightedBackExtensionWithOppositeArmAndLegReach ) , 2 => Ok ( HyperextensionExerciseName :: BaseRotations ) , 3 => Ok ( HyperextensionExerciseName :: WeightedBaseRotations ) , 4 => Ok ( HyperextensionExerciseName :: BentKneeReverseHyperextension ) , 5 => Ok ( HyperextensionExerciseName :: WeightedBentKneeReverseHyperextension ) , 6 => Ok ( HyperextensionExerciseName :: HollowHoldAndRoll ) , 7 => Ok ( HyperextensionExerciseName :: WeightedHollowHoldAndRoll ) , 8 => Ok ( HyperextensionExerciseName :: Kicks ) , 9 => Ok ( HyperextensionExerciseName :: WeightedKicks ) , 10 => Ok ( HyperextensionExerciseName :: KneeRaises ) , 11 => Ok ( HyperextensionExerciseName :: WeightedKneeRaises ) , 12 => Ok ( HyperextensionExerciseName :: KneelingSuperman ) , 13 => Ok ( HyperextensionExerciseName :: WeightedKneelingSuperman ) , 14 => Ok ( HyperextensionExerciseName :: LatPullDownWithRow ) , 15 => Ok ( HyperextensionExerciseName :: MedicineBallDeadliftToReach ) , 16 => Ok ( HyperextensionExerciseName :: OneArmOneLegRow ) , 17 => Ok ( HyperextensionExerciseName :: OneArmRowWithBand ) , 18 => Ok ( HyperextensionExerciseName :: OverheadLungeWithMedicineBall ) , 19 => Ok ( HyperextensionExerciseName :: PlankKneeTucks ) , 20 => Ok ( HyperextensionExerciseName :: WeightedPlankKneeTucks ) , 21 => Ok ( HyperextensionExerciseName :: SideStep ) , 22 => Ok ( HyperextensionExerciseName :: WeightedSideStep ) , 23 => Ok ( HyperextensionExerciseName :: SingleLegBackExtension ) , 24 => Ok ( HyperextensionExerciseName :: WeightedSingleLegBackExtension ) , 25 => Ok ( HyperextensionExerciseName :: SpineExtension ) , 26 => Ok ( HyperextensionExerciseName :: WeightedSpineExtension ) , 27 => Ok ( HyperextensionExerciseName :: StaticBackExtension ) , 28 => Ok ( HyperextensionExerciseName :: WeightedStaticBackExtension ) , 29 => Ok ( HyperextensionExerciseName :: SupermanFromFloor ) , 30 => Ok ( HyperextensionExerciseName :: WeightedSupermanFromFloor ) , 31 => Ok ( HyperextensionExerciseName :: SwissBallBackExtension ) , 32 => Ok ( HyperextensionExerciseName :: WeightedSwissBallBackExtension ) , 33 => Ok ( HyperextensionExerciseName :: SwissBallHyperextension ) , 34 => Ok ( HyperextensionExerciseName :: WeightedSwissBallHyperextension ) , 35 => Ok ( HyperextensionExerciseName :: SwissBallOppositeArmAndLegLift ) , 36 => Ok ( HyperextensionExerciseName :: WeightedSwissBallOppositeArmAndLegLift ) , _ => Ok ( HyperextensionExerciseName :: Unknown ) , }
    }
}
#[derive(Debug)]
pub enum LateralRaiseExerciseName {
    FourtyFiveDegreeCableExternalRotation = 0,
    AlternatingLateralRaiseWithStaticHold = 1,
    BarMuscleUp = 2,
    BentOverLateralRaise = 3,
    CableDiagonalRaise = 4,
    CableFrontRaise = 5,
    CalorieRow = 6,
    ComboShoulderRaise = 7,
    DumbbellDiagonalRaise = 8,
    DumbbellVRaise = 9,
    FrontRaise = 10,
    LeaningDumbbellLateralRaise = 11,
    LyingDumbbellRaise = 12,
    MuscleUp = 13,
    OneArmCableLateralRaise = 14,
    OverhandGripRearLateralRaise = 15,
    PlateRaises = 16,
    RingDip = 17,
    WeightedRingDip = 18,
    RingMuscleUp = 19,
    WeightedRingMuscleUp = 20,
    RopeClimb = 21,
    WeightedRopeClimb = 22,
    Scaption = 23,
    SeatedLateralRaise = 24,
    SeatedRearLateralRaise = 25,
    SideLyingLateralRaise = 26,
    StandingLift = 27,
    SuspendedRow = 28,
    UnderhandGripRearLateralRaise = 29,
    WallSlide = 30,
    WeightedWallSlide = 31,
    Unknown,
}
impl LateralRaiseExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(
                LateralRaiseExerciseName::FourtyFiveDegreeCableExternalRotation,
            ),
            1 => Ok(
                LateralRaiseExerciseName::AlternatingLateralRaiseWithStaticHold,
            ),
            2 => Ok(LateralRaiseExerciseName::BarMuscleUp),
            3 => Ok(LateralRaiseExerciseName::BentOverLateralRaise),
            4 => Ok(LateralRaiseExerciseName::CableDiagonalRaise),
            5 => Ok(LateralRaiseExerciseName::CableFrontRaise),
            6 => Ok(LateralRaiseExerciseName::CalorieRow),
            7 => Ok(LateralRaiseExerciseName::ComboShoulderRaise),
            8 => Ok(LateralRaiseExerciseName::DumbbellDiagonalRaise),
            9 => Ok(LateralRaiseExerciseName::DumbbellVRaise),
            10 => Ok(LateralRaiseExerciseName::FrontRaise),
            11 => Ok(LateralRaiseExerciseName::LeaningDumbbellLateralRaise),
            12 => Ok(LateralRaiseExerciseName::LyingDumbbellRaise),
            13 => Ok(LateralRaiseExerciseName::MuscleUp),
            14 => Ok(LateralRaiseExerciseName::OneArmCableLateralRaise),
            15 => Ok(LateralRaiseExerciseName::OverhandGripRearLateralRaise),
            16 => Ok(LateralRaiseExerciseName::PlateRaises),
            17 => Ok(LateralRaiseExerciseName::RingDip),
            18 => Ok(LateralRaiseExerciseName::WeightedRingDip),
            19 => Ok(LateralRaiseExerciseName::RingMuscleUp),
            20 => Ok(LateralRaiseExerciseName::WeightedRingMuscleUp),
            21 => Ok(LateralRaiseExerciseName::RopeClimb),
            22 => Ok(LateralRaiseExerciseName::WeightedRopeClimb),
            23 => Ok(LateralRaiseExerciseName::Scaption),
            24 => Ok(LateralRaiseExerciseName::SeatedLateralRaise),
            25 => Ok(LateralRaiseExerciseName::SeatedRearLateralRaise),
            26 => Ok(LateralRaiseExerciseName::SideLyingLateralRaise),
            27 => Ok(LateralRaiseExerciseName::StandingLift),
            28 => Ok(LateralRaiseExerciseName::SuspendedRow),
            29 => Ok(LateralRaiseExerciseName::UnderhandGripRearLateralRaise),
            30 => Ok(LateralRaiseExerciseName::WallSlide),
            31 => Ok(LateralRaiseExerciseName::WeightedWallSlide),
            _ => Ok(LateralRaiseExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LegCurlExerciseName {
    LegCurl = 0,
    WeightedLegCurl = 1,
    GoodMorning = 2,
    SeatedBarbellGoodMorning = 3,
    SingleLegBarbellGoodMorning = 4,
    SingleLegSlidingLegCurl = 5,
    SlidingLegCurl = 6,
    SplitBarbellGoodMorning = 7,
    SplitStanceExtension = 8,
    StaggeredStanceGoodMorning = 9,
    SwissBallHipRaiseAndLegCurl = 10,
    ZercherGoodMorning = 11,
    Unknown,
}
impl LegCurlExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(LegCurlExerciseName::LegCurl),
            1 => Ok(LegCurlExerciseName::WeightedLegCurl),
            2 => Ok(LegCurlExerciseName::GoodMorning),
            3 => Ok(LegCurlExerciseName::SeatedBarbellGoodMorning),
            4 => Ok(LegCurlExerciseName::SingleLegBarbellGoodMorning),
            5 => Ok(LegCurlExerciseName::SingleLegSlidingLegCurl),
            6 => Ok(LegCurlExerciseName::SlidingLegCurl),
            7 => Ok(LegCurlExerciseName::SplitBarbellGoodMorning),
            8 => Ok(LegCurlExerciseName::SplitStanceExtension),
            9 => Ok(LegCurlExerciseName::StaggeredStanceGoodMorning),
            10 => Ok(LegCurlExerciseName::SwissBallHipRaiseAndLegCurl),
            11 => Ok(LegCurlExerciseName::ZercherGoodMorning),
            _ => Ok(LegCurlExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LegRaiseExerciseName {
    HangingKneeRaise = 0,
    HangingLegRaise = 1,
    WeightedHangingLegRaise = 2,
    HangingSingleLegRaise = 3,
    WeightedHangingSingleLegRaise = 4,
    KettlebellLegRaises = 5,
    LegLoweringDrill = 6,
    WeightedLegLoweringDrill = 7,
    LyingStraightLegRaise = 8,
    WeightedLyingStraightLegRaise = 9,
    MedicineBallLegDrops = 10,
    QuadrupedLegRaise = 11,
    WeightedQuadrupedLegRaise = 12,
    ReverseLegRaise = 13,
    WeightedReverseLegRaise = 14,
    ReverseLegRaiseOnSwissBall = 15,
    WeightedReverseLegRaiseOnSwissBall = 16,
    SingleLegLoweringDrill = 17,
    WeightedSingleLegLoweringDrill = 18,
    WeightedHangingKneeRaise = 19,
    LateralStepover = 20,
    WeightedLateralStepover = 21,
    Unknown,
}
impl LegRaiseExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(LegRaiseExerciseName::HangingKneeRaise),
            1 => Ok(LegRaiseExerciseName::HangingLegRaise),
            2 => Ok(LegRaiseExerciseName::WeightedHangingLegRaise),
            3 => Ok(LegRaiseExerciseName::HangingSingleLegRaise),
            4 => Ok(LegRaiseExerciseName::WeightedHangingSingleLegRaise),
            5 => Ok(LegRaiseExerciseName::KettlebellLegRaises),
            6 => Ok(LegRaiseExerciseName::LegLoweringDrill),
            7 => Ok(LegRaiseExerciseName::WeightedLegLoweringDrill),
            8 => Ok(LegRaiseExerciseName::LyingStraightLegRaise),
            9 => Ok(LegRaiseExerciseName::WeightedLyingStraightLegRaise),
            10 => Ok(LegRaiseExerciseName::MedicineBallLegDrops),
            11 => Ok(LegRaiseExerciseName::QuadrupedLegRaise),
            12 => Ok(LegRaiseExerciseName::WeightedQuadrupedLegRaise),
            13 => Ok(LegRaiseExerciseName::ReverseLegRaise),
            14 => Ok(LegRaiseExerciseName::WeightedReverseLegRaise),
            15 => Ok(LegRaiseExerciseName::ReverseLegRaiseOnSwissBall),
            16 => Ok(LegRaiseExerciseName::WeightedReverseLegRaiseOnSwissBall),
            17 => Ok(LegRaiseExerciseName::SingleLegLoweringDrill),
            18 => Ok(LegRaiseExerciseName::WeightedSingleLegLoweringDrill),
            19 => Ok(LegRaiseExerciseName::WeightedHangingKneeRaise),
            20 => Ok(LegRaiseExerciseName::LateralStepover),
            21 => Ok(LegRaiseExerciseName::WeightedLateralStepover),
            _ => Ok(LegRaiseExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum LungeExerciseName {
    OverheadLunge = 0,
    LungeMatrix = 1,
    WeightedLungeMatrix = 2,
    AlternatingBarbellForwardLunge = 3,
    AlternatingDumbbellLungeWithReach = 4,
    BackFootElevatedDumbbellSplitSquat = 5,
    BarbellBoxLunge = 6,
    BarbellBulgarianSplitSquat = 7,
    BarbellCrossoverLunge = 8,
    BarbellFrontSplitSquat = 9,
    BarbellLunge = 10,
    BarbellReverseLunge = 11,
    BarbellSideLunge = 12,
    BarbellSplitSquat = 13,
    CoreControlRearLunge = 14,
    DiagonalLunge = 15,
    DropLunge = 16,
    DumbbellBoxLunge = 17,
    DumbbellBulgarianSplitSquat = 18,
    DumbbellCrossoverLunge = 19,
    DumbbellDiagonalLunge = 20,
    DumbbellLunge = 21,
    DumbbellLungeAndRotation = 22,
    DumbbellOverheadBulgarianSplitSquat = 23,
    DumbbellReverseLungeToHighKneeAndPress = 24,
    DumbbellSideLunge = 25,
    ElevatedFrontFootBarbellSplitSquat = 26,
    FrontFootElevatedDumbbellSplitSquat = 27,
    GunslingerLunge = 28,
    LawnmowerLunge = 29,
    LowLungeWithIsometricAdduction = 30,
    LowSideToSideLunge = 31,
    Lunge = 32,
    WeightedLunge = 33,
    LungeWithArmReach = 34,
    LungeWithDiagonalReach = 35,
    LungeWithSideBend = 36,
    OffsetDumbbellLunge = 37,
    OffsetDumbbellReverseLunge = 38,
    OverheadBulgarianSplitSquat = 39,
    OverheadDumbbellReverseLunge = 40,
    OverheadDumbbellSplitSquat = 41,
    OverheadLungeWithRotation = 42,
    ReverseBarbellBoxLunge = 43,
    ReverseBoxLunge = 44,
    ReverseDumbbellBoxLunge = 45,
    ReverseDumbbellCrossoverLunge = 46,
    ReverseDumbbellDiagonalLunge = 47,
    ReverseLungeWithReachBack = 48,
    WeightedReverseLungeWithReachBack = 49,
    ReverseLungeWithTwistAndOverheadReach = 50,
    WeightedReverseLungeWithTwistAndOverheadReach = 51,
    ReverseSlidingBoxLunge = 52,
    WeightedReverseSlidingBoxLunge = 53,
    ReverseSlidingLunge = 54,
    WeightedReverseSlidingLunge = 55,
    RunnersLungeToBalance = 56,
    WeightedRunnersLungeToBalance = 57,
    ShiftingSideLunge = 58,
    SideAndCrossoverLunge = 59,
    WeightedSideAndCrossoverLunge = 60,
    SideLunge = 61,
    WeightedSideLunge = 62,
    SideLungeAndPress = 63,
    SideLungeJumpOff = 64,
    SideLungeSweep = 65,
    WeightedSideLungeSweep = 66,
    SideLungeToCrossoverTap = 67,
    WeightedSideLungeToCrossoverTap = 68,
    SideToSideLungeChops = 69,
    WeightedSideToSideLungeChops = 70,
    SiffJumpLunge = 71,
    WeightedSiffJumpLunge = 72,
    SingleArmReverseLungeAndPress = 73,
    SlidingLateralLunge = 74,
    WeightedSlidingLateralLunge = 75,
    WalkingBarbellLunge = 76,
    WalkingDumbbellLunge = 77,
    WalkingLunge = 78,
    WeightedWalkingLunge = 79,
    WideGripOverheadBarbellSplitSquat = 80,
    Unknown,
}
impl LungeExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value . 0 { 0 => Ok ( LungeExerciseName :: OverheadLunge ) , 1 => Ok ( LungeExerciseName :: LungeMatrix ) , 2 => Ok ( LungeExerciseName :: WeightedLungeMatrix ) , 3 => Ok ( LungeExerciseName :: AlternatingBarbellForwardLunge ) , 4 => Ok ( LungeExerciseName :: AlternatingDumbbellLungeWithReach ) , 5 => Ok ( LungeExerciseName :: BackFootElevatedDumbbellSplitSquat ) , 6 => Ok ( LungeExerciseName :: BarbellBoxLunge ) , 7 => Ok ( LungeExerciseName :: BarbellBulgarianSplitSquat ) , 8 => Ok ( LungeExerciseName :: BarbellCrossoverLunge ) , 9 => Ok ( LungeExerciseName :: BarbellFrontSplitSquat ) , 10 => Ok ( LungeExerciseName :: BarbellLunge ) , 11 => Ok ( LungeExerciseName :: BarbellReverseLunge ) , 12 => Ok ( LungeExerciseName :: BarbellSideLunge ) , 13 => Ok ( LungeExerciseName :: BarbellSplitSquat ) , 14 => Ok ( LungeExerciseName :: CoreControlRearLunge ) , 15 => Ok ( LungeExerciseName :: DiagonalLunge ) , 16 => Ok ( LungeExerciseName :: DropLunge ) , 17 => Ok ( LungeExerciseName :: DumbbellBoxLunge ) , 18 => Ok ( LungeExerciseName :: DumbbellBulgarianSplitSquat ) , 19 => Ok ( LungeExerciseName :: DumbbellCrossoverLunge ) , 20 => Ok ( LungeExerciseName :: DumbbellDiagonalLunge ) , 21 => Ok ( LungeExerciseName :: DumbbellLunge ) , 22 => Ok ( LungeExerciseName :: DumbbellLungeAndRotation ) , 23 => Ok ( LungeExerciseName :: DumbbellOverheadBulgarianSplitSquat ) , 24 => Ok ( LungeExerciseName :: DumbbellReverseLungeToHighKneeAndPress ) , 25 => Ok ( LungeExerciseName :: DumbbellSideLunge ) , 26 => Ok ( LungeExerciseName :: ElevatedFrontFootBarbellSplitSquat ) , 27 => Ok ( LungeExerciseName :: FrontFootElevatedDumbbellSplitSquat ) , 28 => Ok ( LungeExerciseName :: GunslingerLunge ) , 29 => Ok ( LungeExerciseName :: LawnmowerLunge ) , 30 => Ok ( LungeExerciseName :: LowLungeWithIsometricAdduction ) , 31 => Ok ( LungeExerciseName :: LowSideToSideLunge ) , 32 => Ok ( LungeExerciseName :: Lunge ) , 33 => Ok ( LungeExerciseName :: WeightedLunge ) , 34 => Ok ( LungeExerciseName :: LungeWithArmReach ) , 35 => Ok ( LungeExerciseName :: LungeWithDiagonalReach ) , 36 => Ok ( LungeExerciseName :: LungeWithSideBend ) , 37 => Ok ( LungeExerciseName :: OffsetDumbbellLunge ) , 38 => Ok ( LungeExerciseName :: OffsetDumbbellReverseLunge ) , 39 => Ok ( LungeExerciseName :: OverheadBulgarianSplitSquat ) , 40 => Ok ( LungeExerciseName :: OverheadDumbbellReverseLunge ) , 41 => Ok ( LungeExerciseName :: OverheadDumbbellSplitSquat ) , 42 => Ok ( LungeExerciseName :: OverheadLungeWithRotation ) , 43 => Ok ( LungeExerciseName :: ReverseBarbellBoxLunge ) , 44 => Ok ( LungeExerciseName :: ReverseBoxLunge ) , 45 => Ok ( LungeExerciseName :: ReverseDumbbellBoxLunge ) , 46 => Ok ( LungeExerciseName :: ReverseDumbbellCrossoverLunge ) , 47 => Ok ( LungeExerciseName :: ReverseDumbbellDiagonalLunge ) , 48 => Ok ( LungeExerciseName :: ReverseLungeWithReachBack ) , 49 => Ok ( LungeExerciseName :: WeightedReverseLungeWithReachBack ) , 50 => Ok ( LungeExerciseName :: ReverseLungeWithTwistAndOverheadReach ) , 51 => Ok ( LungeExerciseName :: WeightedReverseLungeWithTwistAndOverheadReach ) , 52 => Ok ( LungeExerciseName :: ReverseSlidingBoxLunge ) , 53 => Ok ( LungeExerciseName :: WeightedReverseSlidingBoxLunge ) , 54 => Ok ( LungeExerciseName :: ReverseSlidingLunge ) , 55 => Ok ( LungeExerciseName :: WeightedReverseSlidingLunge ) , 56 => Ok ( LungeExerciseName :: RunnersLungeToBalance ) , 57 => Ok ( LungeExerciseName :: WeightedRunnersLungeToBalance ) , 58 => Ok ( LungeExerciseName :: ShiftingSideLunge ) , 59 => Ok ( LungeExerciseName :: SideAndCrossoverLunge ) , 60 => Ok ( LungeExerciseName :: WeightedSideAndCrossoverLunge ) , 61 => Ok ( LungeExerciseName :: SideLunge ) , 62 => Ok ( LungeExerciseName :: WeightedSideLunge ) , 63 => Ok ( LungeExerciseName :: SideLungeAndPress ) , 64 => Ok ( LungeExerciseName :: SideLungeJumpOff ) , 65 => Ok ( LungeExerciseName :: SideLungeSweep ) , 66 => Ok ( LungeExerciseName :: WeightedSideLungeSweep ) , 67 => Ok ( LungeExerciseName :: SideLungeToCrossoverTap ) , 68 => Ok ( LungeExerciseName :: WeightedSideLungeToCrossoverTap ) , 69 => Ok ( LungeExerciseName :: SideToSideLungeChops ) , 70 => Ok ( LungeExerciseName :: WeightedSideToSideLungeChops ) , 71 => Ok ( LungeExerciseName :: SiffJumpLunge ) , 72 => Ok ( LungeExerciseName :: WeightedSiffJumpLunge ) , 73 => Ok ( LungeExerciseName :: SingleArmReverseLungeAndPress ) , 74 => Ok ( LungeExerciseName :: SlidingLateralLunge ) , 75 => Ok ( LungeExerciseName :: WeightedSlidingLateralLunge ) , 76 => Ok ( LungeExerciseName :: WalkingBarbellLunge ) , 77 => Ok ( LungeExerciseName :: WalkingDumbbellLunge ) , 78 => Ok ( LungeExerciseName :: WalkingLunge ) , 79 => Ok ( LungeExerciseName :: WeightedWalkingLunge ) , 80 => Ok ( LungeExerciseName :: WideGripOverheadBarbellSplitSquat ) , _ => Ok ( LungeExerciseName :: Unknown ) , }
    }
}
#[derive(Debug)]
pub enum OlympicLiftExerciseName {
    BarbellHangPowerClean = 0,
    BarbellHangSquatClean = 1,
    BarbellPowerClean = 2,
    BarbellPowerSnatch = 3,
    BarbellSquatClean = 4,
    CleanAndJerk = 5,
    BarbellHangPowerSnatch = 6,
    BarbellHangPull = 7,
    BarbellHighPull = 8,
    BarbellSnatch = 9,
    BarbellSplitJerk = 10,
    Clean = 11,
    DumbbellClean = 12,
    DumbbellHangPull = 13,
    OneHandDumbbellSplitSnatch = 14,
    PushJerk = 15,
    SingleArmDumbbellSnatch = 16,
    SingleArmHangSnatch = 17,
    SingleArmKettlebellSnatch = 18,
    SplitJerk = 19,
    SquatCleanAndJerk = 20,
    Unknown,
}
impl OlympicLiftExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(OlympicLiftExerciseName::BarbellHangPowerClean),
            1 => Ok(OlympicLiftExerciseName::BarbellHangSquatClean),
            2 => Ok(OlympicLiftExerciseName::BarbellPowerClean),
            3 => Ok(OlympicLiftExerciseName::BarbellPowerSnatch),
            4 => Ok(OlympicLiftExerciseName::BarbellSquatClean),
            5 => Ok(OlympicLiftExerciseName::CleanAndJerk),
            6 => Ok(OlympicLiftExerciseName::BarbellHangPowerSnatch),
            7 => Ok(OlympicLiftExerciseName::BarbellHangPull),
            8 => Ok(OlympicLiftExerciseName::BarbellHighPull),
            9 => Ok(OlympicLiftExerciseName::BarbellSnatch),
            10 => Ok(OlympicLiftExerciseName::BarbellSplitJerk),
            11 => Ok(OlympicLiftExerciseName::Clean),
            12 => Ok(OlympicLiftExerciseName::DumbbellClean),
            13 => Ok(OlympicLiftExerciseName::DumbbellHangPull),
            14 => Ok(OlympicLiftExerciseName::OneHandDumbbellSplitSnatch),
            15 => Ok(OlympicLiftExerciseName::PushJerk),
            16 => Ok(OlympicLiftExerciseName::SingleArmDumbbellSnatch),
            17 => Ok(OlympicLiftExerciseName::SingleArmHangSnatch),
            18 => Ok(OlympicLiftExerciseName::SingleArmKettlebellSnatch),
            19 => Ok(OlympicLiftExerciseName::SplitJerk),
            20 => Ok(OlympicLiftExerciseName::SquatCleanAndJerk),
            _ => Ok(OlympicLiftExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum PlankExerciseName {
    FourtyFiveDegreePlank = 0,
    Weighted45DegreePlank = 1,
    NinetyDegreeStaticHold = 2,
    Weighted90DegreeStaticHold = 3,
    BearCrawl = 4,
    WeightedBearCrawl = 5,
    CrossBodyMountainClimber = 6,
    WeightedCrossBodyMountainClimber = 7,
    ElbowPlankPikeJacks = 8,
    WeightedElbowPlankPikeJacks = 9,
    ElevatedFeetPlank = 10,
    WeightedElevatedFeetPlank = 11,
    ElevatorAbs = 12,
    WeightedElevatorAbs = 13,
    ExtendedPlank = 14,
    WeightedExtendedPlank = 15,
    FullPlankPasseTwist = 16,
    WeightedFullPlankPasseTwist = 17,
    InchingElbowPlank = 18,
    WeightedInchingElbowPlank = 19,
    InchwormToSidePlank = 20,
    WeightedInchwormToSidePlank = 21,
    KneelingPlank = 22,
    WeightedKneelingPlank = 23,
    KneelingSidePlankWithLegLift = 24,
    WeightedKneelingSidePlankWithLegLift = 25,
    LateralRoll = 26,
    WeightedLateralRoll = 27,
    LyingReversePlank = 28,
    WeightedLyingReversePlank = 29,
    MedicineBallMountainClimber = 30,
    WeightedMedicineBallMountainClimber = 31,
    ModifiedMountainClimberAndExtension = 32,
    WeightedModifiedMountainClimberAndExtension = 33,
    MountainClimber = 34,
    WeightedMountainClimber = 35,
    MountainClimberOnSlidingDiscs = 36,
    WeightedMountainClimberOnSlidingDiscs = 37,
    MountainClimberWithFeetOnBosuBall = 38,
    WeightedMountainClimberWithFeetOnBosuBall = 39,
    MountainClimberWithHandsOnBench = 40,
    MountainClimberWithHandsOnSwissBall = 41,
    WeightedMountainClimberWithHandsOnSwissBall = 42,
    Plank = 43,
    PlankJacksWithFeetOnSlidingDiscs = 44,
    WeightedPlankJacksWithFeetOnSlidingDiscs = 45,
    PlankKneeTwist = 46,
    WeightedPlankKneeTwist = 47,
    PlankPikeJumps = 48,
    WeightedPlankPikeJumps = 49,
    PlankPikes = 50,
    WeightedPlankPikes = 51,
    PlankToStandUp = 52,
    WeightedPlankToStandUp = 53,
    PlankWithArmRaise = 54,
    WeightedPlankWithArmRaise = 55,
    PlankWithKneeToElbow = 56,
    WeightedPlankWithKneeToElbow = 57,
    PlankWithObliqueCrunch = 58,
    WeightedPlankWithObliqueCrunch = 59,
    PlyometricSidePlank = 60,
    WeightedPlyometricSidePlank = 61,
    RollingSidePlank = 62,
    WeightedRollingSidePlank = 63,
    SideKickPlank = 64,
    WeightedSideKickPlank = 65,
    SidePlank = 66,
    WeightedSidePlank = 67,
    SidePlankAndRow = 68,
    WeightedSidePlankAndRow = 69,
    SidePlankLift = 70,
    WeightedSidePlankLift = 71,
    SidePlankWithElbowOnBosuBall = 72,
    WeightedSidePlankWithElbowOnBosuBall = 73,
    SidePlankWithFeetOnBench = 74,
    WeightedSidePlankWithFeetOnBench = 75,
    SidePlankWithKneeCircle = 76,
    WeightedSidePlankWithKneeCircle = 77,
    SidePlankWithKneeTuck = 78,
    WeightedSidePlankWithKneeTuck = 79,
    SidePlankWithLegLift = 80,
    WeightedSidePlankWithLegLift = 81,
    SidePlankWithReachUnder = 82,
    WeightedSidePlankWithReachUnder = 83,
    SingleLegElevatedFeetPlank = 84,
    WeightedSingleLegElevatedFeetPlank = 85,
    SingleLegFlexAndExtend = 86,
    WeightedSingleLegFlexAndExtend = 87,
    SingleLegSidePlank = 88,
    WeightedSingleLegSidePlank = 89,
    SpidermanPlank = 90,
    WeightedSpidermanPlank = 91,
    StraightArmPlank = 92,
    WeightedStraightArmPlank = 93,
    StraightArmPlankWithShoulderTouch = 94,
    WeightedStraightArmPlankWithShoulderTouch = 95,
    SwissBallPlank = 96,
    WeightedSwissBallPlank = 97,
    SwissBallPlankLegLift = 98,
    WeightedSwissBallPlankLegLift = 99,
    SwissBallPlankLegLiftAndHold = 100,
    SwissBallPlankWithFeetOnBench = 101,
    WeightedSwissBallPlankWithFeetOnBench = 102,
    SwissBallProneJackknife = 103,
    WeightedSwissBallProneJackknife = 104,
    SwissBallSidePlank = 105,
    WeightedSwissBallSidePlank = 106,
    ThreeWayPlank = 107,
    WeightedThreeWayPlank = 108,
    TowelPlankAndKneeIn = 109,
    WeightedTowelPlankAndKneeIn = 110,
    TStabilization = 111,
    WeightedTStabilization = 112,
    TurkishGetUpToSidePlank = 113,
    WeightedTurkishGetUpToSidePlank = 114,
    TwoPointPlank = 115,
    WeightedTwoPointPlank = 116,
    WeightedPlank = 117,
    WideStancePlankWithDiagonalArmLift = 118,
    WeightedWideStancePlankWithDiagonalArmLift = 119,
    WideStancePlankWithDiagonalLegLift = 120,
    WeightedWideStancePlankWithDiagonalLegLift = 121,
    WideStancePlankWithLegLift = 122,
    WeightedWideStancePlankWithLegLift = 123,
    WideStancePlankWithOppositeArmAndLegLift = 124,
    WeightedMountainClimberWithHandsOnBench = 125,
    WeightedSwissBallPlankLegLiftAndHold = 126,
    WeightedWideStancePlankWithOppositeArmAndLegLift = 127,
    Unknown,
}
impl PlankExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value . 0 { 0 => Ok ( PlankExerciseName :: FourtyFiveDegreePlank ) , 1 => Ok ( PlankExerciseName :: Weighted45DegreePlank ) , 2 => Ok ( PlankExerciseName :: NinetyDegreeStaticHold ) , 3 => Ok ( PlankExerciseName :: Weighted90DegreeStaticHold ) , 4 => Ok ( PlankExerciseName :: BearCrawl ) , 5 => Ok ( PlankExerciseName :: WeightedBearCrawl ) , 6 => Ok ( PlankExerciseName :: CrossBodyMountainClimber ) , 7 => Ok ( PlankExerciseName :: WeightedCrossBodyMountainClimber ) , 8 => Ok ( PlankExerciseName :: ElbowPlankPikeJacks ) , 9 => Ok ( PlankExerciseName :: WeightedElbowPlankPikeJacks ) , 10 => Ok ( PlankExerciseName :: ElevatedFeetPlank ) , 11 => Ok ( PlankExerciseName :: WeightedElevatedFeetPlank ) , 12 => Ok ( PlankExerciseName :: ElevatorAbs ) , 13 => Ok ( PlankExerciseName :: WeightedElevatorAbs ) , 14 => Ok ( PlankExerciseName :: ExtendedPlank ) , 15 => Ok ( PlankExerciseName :: WeightedExtendedPlank ) , 16 => Ok ( PlankExerciseName :: FullPlankPasseTwist ) , 17 => Ok ( PlankExerciseName :: WeightedFullPlankPasseTwist ) , 18 => Ok ( PlankExerciseName :: InchingElbowPlank ) , 19 => Ok ( PlankExerciseName :: WeightedInchingElbowPlank ) , 20 => Ok ( PlankExerciseName :: InchwormToSidePlank ) , 21 => Ok ( PlankExerciseName :: WeightedInchwormToSidePlank ) , 22 => Ok ( PlankExerciseName :: KneelingPlank ) , 23 => Ok ( PlankExerciseName :: WeightedKneelingPlank ) , 24 => Ok ( PlankExerciseName :: KneelingSidePlankWithLegLift ) , 25 => Ok ( PlankExerciseName :: WeightedKneelingSidePlankWithLegLift ) , 26 => Ok ( PlankExerciseName :: LateralRoll ) , 27 => Ok ( PlankExerciseName :: WeightedLateralRoll ) , 28 => Ok ( PlankExerciseName :: LyingReversePlank ) , 29 => Ok ( PlankExerciseName :: WeightedLyingReversePlank ) , 30 => Ok ( PlankExerciseName :: MedicineBallMountainClimber ) , 31 => Ok ( PlankExerciseName :: WeightedMedicineBallMountainClimber ) , 32 => Ok ( PlankExerciseName :: ModifiedMountainClimberAndExtension ) , 33 => Ok ( PlankExerciseName :: WeightedModifiedMountainClimberAndExtension ) , 34 => Ok ( PlankExerciseName :: MountainClimber ) , 35 => Ok ( PlankExerciseName :: WeightedMountainClimber ) , 36 => Ok ( PlankExerciseName :: MountainClimberOnSlidingDiscs ) , 37 => Ok ( PlankExerciseName :: WeightedMountainClimberOnSlidingDiscs ) , 38 => Ok ( PlankExerciseName :: MountainClimberWithFeetOnBosuBall ) , 39 => Ok ( PlankExerciseName :: WeightedMountainClimberWithFeetOnBosuBall ) , 40 => Ok ( PlankExerciseName :: MountainClimberWithHandsOnBench ) , 41 => Ok ( PlankExerciseName :: MountainClimberWithHandsOnSwissBall ) , 42 => Ok ( PlankExerciseName :: WeightedMountainClimberWithHandsOnSwissBall ) , 43 => Ok ( PlankExerciseName :: Plank ) , 44 => Ok ( PlankExerciseName :: PlankJacksWithFeetOnSlidingDiscs ) , 45 => Ok ( PlankExerciseName :: WeightedPlankJacksWithFeetOnSlidingDiscs ) , 46 => Ok ( PlankExerciseName :: PlankKneeTwist ) , 47 => Ok ( PlankExerciseName :: WeightedPlankKneeTwist ) , 48 => Ok ( PlankExerciseName :: PlankPikeJumps ) , 49 => Ok ( PlankExerciseName :: WeightedPlankPikeJumps ) , 50 => Ok ( PlankExerciseName :: PlankPikes ) , 51 => Ok ( PlankExerciseName :: WeightedPlankPikes ) , 52 => Ok ( PlankExerciseName :: PlankToStandUp ) , 53 => Ok ( PlankExerciseName :: WeightedPlankToStandUp ) , 54 => Ok ( PlankExerciseName :: PlankWithArmRaise ) , 55 => Ok ( PlankExerciseName :: WeightedPlankWithArmRaise ) , 56 => Ok ( PlankExerciseName :: PlankWithKneeToElbow ) , 57 => Ok ( PlankExerciseName :: WeightedPlankWithKneeToElbow ) , 58 => Ok ( PlankExerciseName :: PlankWithObliqueCrunch ) , 59 => Ok ( PlankExerciseName :: WeightedPlankWithObliqueCrunch ) , 60 => Ok ( PlankExerciseName :: PlyometricSidePlank ) , 61 => Ok ( PlankExerciseName :: WeightedPlyometricSidePlank ) , 62 => Ok ( PlankExerciseName :: RollingSidePlank ) , 63 => Ok ( PlankExerciseName :: WeightedRollingSidePlank ) , 64 => Ok ( PlankExerciseName :: SideKickPlank ) , 65 => Ok ( PlankExerciseName :: WeightedSideKickPlank ) , 66 => Ok ( PlankExerciseName :: SidePlank ) , 67 => Ok ( PlankExerciseName :: WeightedSidePlank ) , 68 => Ok ( PlankExerciseName :: SidePlankAndRow ) , 69 => Ok ( PlankExerciseName :: WeightedSidePlankAndRow ) , 70 => Ok ( PlankExerciseName :: SidePlankLift ) , 71 => Ok ( PlankExerciseName :: WeightedSidePlankLift ) , 72 => Ok ( PlankExerciseName :: SidePlankWithElbowOnBosuBall ) , 73 => Ok ( PlankExerciseName :: WeightedSidePlankWithElbowOnBosuBall ) , 74 => Ok ( PlankExerciseName :: SidePlankWithFeetOnBench ) , 75 => Ok ( PlankExerciseName :: WeightedSidePlankWithFeetOnBench ) , 76 => Ok ( PlankExerciseName :: SidePlankWithKneeCircle ) , 77 => Ok ( PlankExerciseName :: WeightedSidePlankWithKneeCircle ) , 78 => Ok ( PlankExerciseName :: SidePlankWithKneeTuck ) , 79 => Ok ( PlankExerciseName :: WeightedSidePlankWithKneeTuck ) , 80 => Ok ( PlankExerciseName :: SidePlankWithLegLift ) , 81 => Ok ( PlankExerciseName :: WeightedSidePlankWithLegLift ) , 82 => Ok ( PlankExerciseName :: SidePlankWithReachUnder ) , 83 => Ok ( PlankExerciseName :: WeightedSidePlankWithReachUnder ) , 84 => Ok ( PlankExerciseName :: SingleLegElevatedFeetPlank ) , 85 => Ok ( PlankExerciseName :: WeightedSingleLegElevatedFeetPlank ) , 86 => Ok ( PlankExerciseName :: SingleLegFlexAndExtend ) , 87 => Ok ( PlankExerciseName :: WeightedSingleLegFlexAndExtend ) , 88 => Ok ( PlankExerciseName :: SingleLegSidePlank ) , 89 => Ok ( PlankExerciseName :: WeightedSingleLegSidePlank ) , 90 => Ok ( PlankExerciseName :: SpidermanPlank ) , 91 => Ok ( PlankExerciseName :: WeightedSpidermanPlank ) , 92 => Ok ( PlankExerciseName :: StraightArmPlank ) , 93 => Ok ( PlankExerciseName :: WeightedStraightArmPlank ) , 94 => Ok ( PlankExerciseName :: StraightArmPlankWithShoulderTouch ) , 95 => Ok ( PlankExerciseName :: WeightedStraightArmPlankWithShoulderTouch ) , 96 => Ok ( PlankExerciseName :: SwissBallPlank ) , 97 => Ok ( PlankExerciseName :: WeightedSwissBallPlank ) , 98 => Ok ( PlankExerciseName :: SwissBallPlankLegLift ) , 99 => Ok ( PlankExerciseName :: WeightedSwissBallPlankLegLift ) , 100 => Ok ( PlankExerciseName :: SwissBallPlankLegLiftAndHold ) , 101 => Ok ( PlankExerciseName :: SwissBallPlankWithFeetOnBench ) , 102 => Ok ( PlankExerciseName :: WeightedSwissBallPlankWithFeetOnBench ) , 103 => Ok ( PlankExerciseName :: SwissBallProneJackknife ) , 104 => Ok ( PlankExerciseName :: WeightedSwissBallProneJackknife ) , 105 => Ok ( PlankExerciseName :: SwissBallSidePlank ) , 106 => Ok ( PlankExerciseName :: WeightedSwissBallSidePlank ) , 107 => Ok ( PlankExerciseName :: ThreeWayPlank ) , 108 => Ok ( PlankExerciseName :: WeightedThreeWayPlank ) , 109 => Ok ( PlankExerciseName :: TowelPlankAndKneeIn ) , 110 => Ok ( PlankExerciseName :: WeightedTowelPlankAndKneeIn ) , 111 => Ok ( PlankExerciseName :: TStabilization ) , 112 => Ok ( PlankExerciseName :: WeightedTStabilization ) , 113 => Ok ( PlankExerciseName :: TurkishGetUpToSidePlank ) , 114 => Ok ( PlankExerciseName :: WeightedTurkishGetUpToSidePlank ) , 115 => Ok ( PlankExerciseName :: TwoPointPlank ) , 116 => Ok ( PlankExerciseName :: WeightedTwoPointPlank ) , 117 => Ok ( PlankExerciseName :: WeightedPlank ) , 118 => Ok ( PlankExerciseName :: WideStancePlankWithDiagonalArmLift ) , 119 => Ok ( PlankExerciseName :: WeightedWideStancePlankWithDiagonalArmLift ) , 120 => Ok ( PlankExerciseName :: WideStancePlankWithDiagonalLegLift ) , 121 => Ok ( PlankExerciseName :: WeightedWideStancePlankWithDiagonalLegLift ) , 122 => Ok ( PlankExerciseName :: WideStancePlankWithLegLift ) , 123 => Ok ( PlankExerciseName :: WeightedWideStancePlankWithLegLift ) , 124 => Ok ( PlankExerciseName :: WideStancePlankWithOppositeArmAndLegLift ) , 125 => Ok ( PlankExerciseName :: WeightedMountainClimberWithHandsOnBench ) , 126 => Ok ( PlankExerciseName :: WeightedSwissBallPlankLegLiftAndHold ) , 127 => Ok ( PlankExerciseName :: WeightedWideStancePlankWithOppositeArmAndLegLift ) , _ => Ok ( PlankExerciseName :: Unknown ) , }
    }
}
#[derive(Debug)]
pub enum PlyoExerciseName {
    AlternatingJumpLunge = 0,
    WeightedAlternatingJumpLunge = 1,
    BarbellJumpSquat = 2,
    BodyWeightJumpSquat = 3,
    WeightedJumpSquat = 4,
    CrossKneeStrike = 5,
    WeightedCrossKneeStrike = 6,
    DepthJump = 7,
    WeightedDepthJump = 8,
    DumbbellJumpSquat = 9,
    DumbbellSplitJump = 10,
    FrontKneeStrike = 11,
    WeightedFrontKneeStrike = 12,
    HighBoxJump = 13,
    WeightedHighBoxJump = 14,
    IsometricExplosiveBodyWeightJumpSquat = 15,
    WeightedIsometricExplosiveJumpSquat = 16,
    LateralLeapAndHop = 17,
    WeightedLateralLeapAndHop = 18,
    LateralPlyoSquats = 19,
    WeightedLateralPlyoSquats = 20,
    LateralSlide = 21,
    WeightedLateralSlide = 22,
    MedicineBallOverheadThrows = 23,
    MedicineBallSideThrow = 24,
    MedicineBallSlam = 25,
    SideToSideMedicineBallThrows = 26,
    SideToSideShuffleJump = 27,
    WeightedSideToSideShuffleJump = 28,
    SquatJumpOntoBox = 29,
    WeightedSquatJumpOntoBox = 30,
    SquatJumpsInAndOut = 31,
    WeightedSquatJumpsInAndOut = 32,
    Unknown,
}
impl PlyoExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(PlyoExerciseName::AlternatingJumpLunge),
            1 => Ok(PlyoExerciseName::WeightedAlternatingJumpLunge),
            2 => Ok(PlyoExerciseName::BarbellJumpSquat),
            3 => Ok(PlyoExerciseName::BodyWeightJumpSquat),
            4 => Ok(PlyoExerciseName::WeightedJumpSquat),
            5 => Ok(PlyoExerciseName::CrossKneeStrike),
            6 => Ok(PlyoExerciseName::WeightedCrossKneeStrike),
            7 => Ok(PlyoExerciseName::DepthJump),
            8 => Ok(PlyoExerciseName::WeightedDepthJump),
            9 => Ok(PlyoExerciseName::DumbbellJumpSquat),
            10 => Ok(PlyoExerciseName::DumbbellSplitJump),
            11 => Ok(PlyoExerciseName::FrontKneeStrike),
            12 => Ok(PlyoExerciseName::WeightedFrontKneeStrike),
            13 => Ok(PlyoExerciseName::HighBoxJump),
            14 => Ok(PlyoExerciseName::WeightedHighBoxJump),
            15 => Ok(PlyoExerciseName::IsometricExplosiveBodyWeightJumpSquat),
            16 => Ok(PlyoExerciseName::WeightedIsometricExplosiveJumpSquat),
            17 => Ok(PlyoExerciseName::LateralLeapAndHop),
            18 => Ok(PlyoExerciseName::WeightedLateralLeapAndHop),
            19 => Ok(PlyoExerciseName::LateralPlyoSquats),
            20 => Ok(PlyoExerciseName::WeightedLateralPlyoSquats),
            21 => Ok(PlyoExerciseName::LateralSlide),
            22 => Ok(PlyoExerciseName::WeightedLateralSlide),
            23 => Ok(PlyoExerciseName::MedicineBallOverheadThrows),
            24 => Ok(PlyoExerciseName::MedicineBallSideThrow),
            25 => Ok(PlyoExerciseName::MedicineBallSlam),
            26 => Ok(PlyoExerciseName::SideToSideMedicineBallThrows),
            27 => Ok(PlyoExerciseName::SideToSideShuffleJump),
            28 => Ok(PlyoExerciseName::WeightedSideToSideShuffleJump),
            29 => Ok(PlyoExerciseName::SquatJumpOntoBox),
            30 => Ok(PlyoExerciseName::WeightedSquatJumpOntoBox),
            31 => Ok(PlyoExerciseName::SquatJumpsInAndOut),
            32 => Ok(PlyoExerciseName::WeightedSquatJumpsInAndOut),
            _ => Ok(PlyoExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum PullUpExerciseName {
    BandedPullUps = 0,
    ThirtyDegreeLatPulldown = 1,
    BandAssistedChinUp = 2,
    CloseGripChinUp = 3,
    WeightedCloseGripChinUp = 4,
    CloseGripLatPulldown = 5,
    CrossoverChinUp = 6,
    WeightedCrossoverChinUp = 7,
    EzBarPullover = 8,
    HangingHurdle = 9,
    WeightedHangingHurdle = 10,
    KneelingLatPulldown = 11,
    KneelingUnderhandGripLatPulldown = 12,
    LatPulldown = 13,
    MixedGripChinUp = 14,
    WeightedMixedGripChinUp = 15,
    MixedGripPullUp = 16,
    WeightedMixedGripPullUp = 17,
    ReverseGripPulldown = 18,
    StandingCablePullover = 19,
    StraightArmPulldown = 20,
    SwissBallEzBarPullover = 21,
    TowelPullUp = 22,
    WeightedTowelPullUp = 23,
    WeightedPullUp = 24,
    WideGripLatPulldown = 25,
    WideGripPullUp = 26,
    WeightedWideGripPullUp = 27,
    BurpeePullUp = 28,
    WeightedBurpeePullUp = 29,
    JumpingPullUps = 30,
    WeightedJumpingPullUps = 31,
    KippingPullUp = 32,
    WeightedKippingPullUp = 33,
    LPullUp = 34,
    WeightedLPullUp = 35,
    SuspendedChinUp = 36,
    WeightedSuspendedChinUp = 37,
    PullUp = 38,
    Unknown,
}
impl PullUpExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(PullUpExerciseName::BandedPullUps),
            1 => Ok(PullUpExerciseName::ThirtyDegreeLatPulldown),
            2 => Ok(PullUpExerciseName::BandAssistedChinUp),
            3 => Ok(PullUpExerciseName::CloseGripChinUp),
            4 => Ok(PullUpExerciseName::WeightedCloseGripChinUp),
            5 => Ok(PullUpExerciseName::CloseGripLatPulldown),
            6 => Ok(PullUpExerciseName::CrossoverChinUp),
            7 => Ok(PullUpExerciseName::WeightedCrossoverChinUp),
            8 => Ok(PullUpExerciseName::EzBarPullover),
            9 => Ok(PullUpExerciseName::HangingHurdle),
            10 => Ok(PullUpExerciseName::WeightedHangingHurdle),
            11 => Ok(PullUpExerciseName::KneelingLatPulldown),
            12 => Ok(PullUpExerciseName::KneelingUnderhandGripLatPulldown),
            13 => Ok(PullUpExerciseName::LatPulldown),
            14 => Ok(PullUpExerciseName::MixedGripChinUp),
            15 => Ok(PullUpExerciseName::WeightedMixedGripChinUp),
            16 => Ok(PullUpExerciseName::MixedGripPullUp),
            17 => Ok(PullUpExerciseName::WeightedMixedGripPullUp),
            18 => Ok(PullUpExerciseName::ReverseGripPulldown),
            19 => Ok(PullUpExerciseName::StandingCablePullover),
            20 => Ok(PullUpExerciseName::StraightArmPulldown),
            21 => Ok(PullUpExerciseName::SwissBallEzBarPullover),
            22 => Ok(PullUpExerciseName::TowelPullUp),
            23 => Ok(PullUpExerciseName::WeightedTowelPullUp),
            24 => Ok(PullUpExerciseName::WeightedPullUp),
            25 => Ok(PullUpExerciseName::WideGripLatPulldown),
            26 => Ok(PullUpExerciseName::WideGripPullUp),
            27 => Ok(PullUpExerciseName::WeightedWideGripPullUp),
            28 => Ok(PullUpExerciseName::BurpeePullUp),
            29 => Ok(PullUpExerciseName::WeightedBurpeePullUp),
            30 => Ok(PullUpExerciseName::JumpingPullUps),
            31 => Ok(PullUpExerciseName::WeightedJumpingPullUps),
            32 => Ok(PullUpExerciseName::KippingPullUp),
            33 => Ok(PullUpExerciseName::WeightedKippingPullUp),
            34 => Ok(PullUpExerciseName::LPullUp),
            35 => Ok(PullUpExerciseName::WeightedLPullUp),
            36 => Ok(PullUpExerciseName::SuspendedChinUp),
            37 => Ok(PullUpExerciseName::WeightedSuspendedChinUp),
            38 => Ok(PullUpExerciseName::PullUp),
            _ => Ok(PullUpExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum PushUpExerciseName {
    ChestPressWithBand = 0,
    AlternatingStaggeredPushUp = 1,
    WeightedAlternatingStaggeredPushUp = 2,
    AlternatingHandsMedicineBallPushUp = 3,
    WeightedAlternatingHandsMedicineBallPushUp = 4,
    BosuBallPushUp = 5,
    WeightedBosuBallPushUp = 6,
    ClappingPushUp = 7,
    WeightedClappingPushUp = 8,
    CloseGripMedicineBallPushUp = 9,
    WeightedCloseGripMedicineBallPushUp = 10,
    CloseHandsPushUp = 11,
    WeightedCloseHandsPushUp = 12,
    DeclinePushUp = 13,
    WeightedDeclinePushUp = 14,
    DiamondPushUp = 15,
    WeightedDiamondPushUp = 16,
    ExplosiveCrossoverPushUp = 17,
    WeightedExplosiveCrossoverPushUp = 18,
    ExplosivePushUp = 19,
    WeightedExplosivePushUp = 20,
    FeetElevatedSideToSidePushUp = 21,
    WeightedFeetElevatedSideToSidePushUp = 22,
    HandReleasePushUp = 23,
    WeightedHandReleasePushUp = 24,
    HandstandPushUp = 25,
    WeightedHandstandPushUp = 26,
    InclinePushUp = 27,
    WeightedInclinePushUp = 28,
    IsometricExplosivePushUp = 29,
    WeightedIsometricExplosivePushUp = 30,
    JudoPushUp = 31,
    WeightedJudoPushUp = 32,
    KneelingPushUp = 33,
    WeightedKneelingPushUp = 34,
    MedicineBallChestPass = 35,
    MedicineBallPushUp = 36,
    WeightedMedicineBallPushUp = 37,
    OneArmPushUp = 38,
    WeightedOneArmPushUp = 39,
    WeightedPushUp = 40,
    PushUpAndRow = 41,
    WeightedPushUpAndRow = 42,
    PushUpPlus = 43,
    WeightedPushUpPlus = 44,
    PushUpWithFeetOnSwissBall = 45,
    WeightedPushUpWithFeetOnSwissBall = 46,
    PushUpWithOneHandOnMedicineBall = 47,
    WeightedPushUpWithOneHandOnMedicineBall = 48,
    ShoulderPushUp = 49,
    WeightedShoulderPushUp = 50,
    SingleArmMedicineBallPushUp = 51,
    WeightedSingleArmMedicineBallPushUp = 52,
    SpidermanPushUp = 53,
    WeightedSpidermanPushUp = 54,
    StackedFeetPushUp = 55,
    WeightedStackedFeetPushUp = 56,
    StaggeredHandsPushUp = 57,
    WeightedStaggeredHandsPushUp = 58,
    SuspendedPushUp = 59,
    WeightedSuspendedPushUp = 60,
    SwissBallPushUp = 61,
    WeightedSwissBallPushUp = 62,
    SwissBallPushUpPlus = 63,
    WeightedSwissBallPushUpPlus = 64,
    TPushUp = 65,
    WeightedTPushUp = 66,
    TripleStopPushUp = 67,
    WeightedTripleStopPushUp = 68,
    WideHandsPushUp = 69,
    WeightedWideHandsPushUp = 70,
    ParalletteHandstandPushUp = 71,
    WeightedParalletteHandstandPushUp = 72,
    RingHandstandPushUp = 73,
    WeightedRingHandstandPushUp = 74,
    RingPushUp = 75,
    WeightedRingPushUp = 76,
    PushUp = 77,
    Unknown,
}
impl PushUpExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(PushUpExerciseName::ChestPressWithBand),
            1 => Ok(PushUpExerciseName::AlternatingStaggeredPushUp),
            2 => Ok(PushUpExerciseName::WeightedAlternatingStaggeredPushUp),
            3 => Ok(PushUpExerciseName::AlternatingHandsMedicineBallPushUp),
            4 => Ok(
                PushUpExerciseName::WeightedAlternatingHandsMedicineBallPushUp,
            ),
            5 => Ok(PushUpExerciseName::BosuBallPushUp),
            6 => Ok(PushUpExerciseName::WeightedBosuBallPushUp),
            7 => Ok(PushUpExerciseName::ClappingPushUp),
            8 => Ok(PushUpExerciseName::WeightedClappingPushUp),
            9 => Ok(PushUpExerciseName::CloseGripMedicineBallPushUp),
            10 => Ok(PushUpExerciseName::WeightedCloseGripMedicineBallPushUp),
            11 => Ok(PushUpExerciseName::CloseHandsPushUp),
            12 => Ok(PushUpExerciseName::WeightedCloseHandsPushUp),
            13 => Ok(PushUpExerciseName::DeclinePushUp),
            14 => Ok(PushUpExerciseName::WeightedDeclinePushUp),
            15 => Ok(PushUpExerciseName::DiamondPushUp),
            16 => Ok(PushUpExerciseName::WeightedDiamondPushUp),
            17 => Ok(PushUpExerciseName::ExplosiveCrossoverPushUp),
            18 => Ok(PushUpExerciseName::WeightedExplosiveCrossoverPushUp),
            19 => Ok(PushUpExerciseName::ExplosivePushUp),
            20 => Ok(PushUpExerciseName::WeightedExplosivePushUp),
            21 => Ok(PushUpExerciseName::FeetElevatedSideToSidePushUp),
            22 => Ok(PushUpExerciseName::WeightedFeetElevatedSideToSidePushUp),
            23 => Ok(PushUpExerciseName::HandReleasePushUp),
            24 => Ok(PushUpExerciseName::WeightedHandReleasePushUp),
            25 => Ok(PushUpExerciseName::HandstandPushUp),
            26 => Ok(PushUpExerciseName::WeightedHandstandPushUp),
            27 => Ok(PushUpExerciseName::InclinePushUp),
            28 => Ok(PushUpExerciseName::WeightedInclinePushUp),
            29 => Ok(PushUpExerciseName::IsometricExplosivePushUp),
            30 => Ok(PushUpExerciseName::WeightedIsometricExplosivePushUp),
            31 => Ok(PushUpExerciseName::JudoPushUp),
            32 => Ok(PushUpExerciseName::WeightedJudoPushUp),
            33 => Ok(PushUpExerciseName::KneelingPushUp),
            34 => Ok(PushUpExerciseName::WeightedKneelingPushUp),
            35 => Ok(PushUpExerciseName::MedicineBallChestPass),
            36 => Ok(PushUpExerciseName::MedicineBallPushUp),
            37 => Ok(PushUpExerciseName::WeightedMedicineBallPushUp),
            38 => Ok(PushUpExerciseName::OneArmPushUp),
            39 => Ok(PushUpExerciseName::WeightedOneArmPushUp),
            40 => Ok(PushUpExerciseName::WeightedPushUp),
            41 => Ok(PushUpExerciseName::PushUpAndRow),
            42 => Ok(PushUpExerciseName::WeightedPushUpAndRow),
            43 => Ok(PushUpExerciseName::PushUpPlus),
            44 => Ok(PushUpExerciseName::WeightedPushUpPlus),
            45 => Ok(PushUpExerciseName::PushUpWithFeetOnSwissBall),
            46 => Ok(PushUpExerciseName::WeightedPushUpWithFeetOnSwissBall),
            47 => Ok(PushUpExerciseName::PushUpWithOneHandOnMedicineBall),
            48 => {
                Ok(PushUpExerciseName::WeightedPushUpWithOneHandOnMedicineBall)
            },
            49 => Ok(PushUpExerciseName::ShoulderPushUp),
            50 => Ok(PushUpExerciseName::WeightedShoulderPushUp),
            51 => Ok(PushUpExerciseName::SingleArmMedicineBallPushUp),
            52 => Ok(PushUpExerciseName::WeightedSingleArmMedicineBallPushUp),
            53 => Ok(PushUpExerciseName::SpidermanPushUp),
            54 => Ok(PushUpExerciseName::WeightedSpidermanPushUp),
            55 => Ok(PushUpExerciseName::StackedFeetPushUp),
            56 => Ok(PushUpExerciseName::WeightedStackedFeetPushUp),
            57 => Ok(PushUpExerciseName::StaggeredHandsPushUp),
            58 => Ok(PushUpExerciseName::WeightedStaggeredHandsPushUp),
            59 => Ok(PushUpExerciseName::SuspendedPushUp),
            60 => Ok(PushUpExerciseName::WeightedSuspendedPushUp),
            61 => Ok(PushUpExerciseName::SwissBallPushUp),
            62 => Ok(PushUpExerciseName::WeightedSwissBallPushUp),
            63 => Ok(PushUpExerciseName::SwissBallPushUpPlus),
            64 => Ok(PushUpExerciseName::WeightedSwissBallPushUpPlus),
            65 => Ok(PushUpExerciseName::TPushUp),
            66 => Ok(PushUpExerciseName::WeightedTPushUp),
            67 => Ok(PushUpExerciseName::TripleStopPushUp),
            68 => Ok(PushUpExerciseName::WeightedTripleStopPushUp),
            69 => Ok(PushUpExerciseName::WideHandsPushUp),
            70 => Ok(PushUpExerciseName::WeightedWideHandsPushUp),
            71 => Ok(PushUpExerciseName::ParalletteHandstandPushUp),
            72 => Ok(PushUpExerciseName::WeightedParalletteHandstandPushUp),
            73 => Ok(PushUpExerciseName::RingHandstandPushUp),
            74 => Ok(PushUpExerciseName::WeightedRingHandstandPushUp),
            75 => Ok(PushUpExerciseName::RingPushUp),
            76 => Ok(PushUpExerciseName::WeightedRingPushUp),
            77 => Ok(PushUpExerciseName::PushUp),
            _ => Ok(PushUpExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum RowExerciseName {
    BarbellStraightLegDeadliftToRow = 0,
    CableRowStanding = 1,
    DumbbellRow = 2,
    ElevatedFeetInvertedRow = 3,
    WeightedElevatedFeetInvertedRow = 4,
    FacePull = 5,
    FacePullWithExternalRotation = 6,
    InvertedRowWithFeetOnSwissBall = 7,
    WeightedInvertedRowWithFeetOnSwissBall = 8,
    KettlebellRow = 9,
    ModifiedInvertedRow = 10,
    WeightedModifiedInvertedRow = 11,
    NeutralGripAlternatingDumbbellRow = 12,
    OneArmBentOverRow = 13,
    OneLeggedDumbbellRow = 14,
    RenegadeRow = 15,
    ReverseGripBarbellRow = 16,
    RopeHandleCableRow = 17,
    SeatedCableRow = 18,
    SeatedDumbbellRow = 19,
    SingleArmCableRow = 20,
    SingleArmCableRowAndRotation = 21,
    SingleArmInvertedRow = 22,
    WeightedSingleArmInvertedRow = 23,
    SingleArmNeutralGripDumbbellRow = 24,
    SingleArmNeutralGripDumbbellRowAndRotation = 25,
    SuspendedInvertedRow = 26,
    WeightedSuspendedInvertedRow = 27,
    TBarRow = 28,
    TowelGripInvertedRow = 29,
    WeightedTowelGripInvertedRow = 30,
    UnderhandGripCableRow = 31,
    VGripCableRow = 32,
    WideGripSeatedCableRow = 33,
    Unknown,
}
impl RowExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(RowExerciseName::BarbellStraightLegDeadliftToRow),
            1 => Ok(RowExerciseName::CableRowStanding),
            2 => Ok(RowExerciseName::DumbbellRow),
            3 => Ok(RowExerciseName::ElevatedFeetInvertedRow),
            4 => Ok(RowExerciseName::WeightedElevatedFeetInvertedRow),
            5 => Ok(RowExerciseName::FacePull),
            6 => Ok(RowExerciseName::FacePullWithExternalRotation),
            7 => Ok(RowExerciseName::InvertedRowWithFeetOnSwissBall),
            8 => Ok(RowExerciseName::WeightedInvertedRowWithFeetOnSwissBall),
            9 => Ok(RowExerciseName::KettlebellRow),
            10 => Ok(RowExerciseName::ModifiedInvertedRow),
            11 => Ok(RowExerciseName::WeightedModifiedInvertedRow),
            12 => Ok(RowExerciseName::NeutralGripAlternatingDumbbellRow),
            13 => Ok(RowExerciseName::OneArmBentOverRow),
            14 => Ok(RowExerciseName::OneLeggedDumbbellRow),
            15 => Ok(RowExerciseName::RenegadeRow),
            16 => Ok(RowExerciseName::ReverseGripBarbellRow),
            17 => Ok(RowExerciseName::RopeHandleCableRow),
            18 => Ok(RowExerciseName::SeatedCableRow),
            19 => Ok(RowExerciseName::SeatedDumbbellRow),
            20 => Ok(RowExerciseName::SingleArmCableRow),
            21 => Ok(RowExerciseName::SingleArmCableRowAndRotation),
            22 => Ok(RowExerciseName::SingleArmInvertedRow),
            23 => Ok(RowExerciseName::WeightedSingleArmInvertedRow),
            24 => Ok(RowExerciseName::SingleArmNeutralGripDumbbellRow),
            25 => {
                Ok(RowExerciseName::SingleArmNeutralGripDumbbellRowAndRotation)
            },
            26 => Ok(RowExerciseName::SuspendedInvertedRow),
            27 => Ok(RowExerciseName::WeightedSuspendedInvertedRow),
            28 => Ok(RowExerciseName::TBarRow),
            29 => Ok(RowExerciseName::TowelGripInvertedRow),
            30 => Ok(RowExerciseName::WeightedTowelGripInvertedRow),
            31 => Ok(RowExerciseName::UnderhandGripCableRow),
            32 => Ok(RowExerciseName::VGripCableRow),
            33 => Ok(RowExerciseName::WideGripSeatedCableRow),
            _ => Ok(RowExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum ShoulderPressExerciseName {
    AlternatingDumbbellShoulderPress = 0,
    ArnoldPress = 1,
    BarbellFrontSquatToPushPress = 2,
    BarbellPushPress = 3,
    BarbellShoulderPress = 4,
    DeadCurlPress = 5,
    DumbbellAlternatingShoulderPressAndTwist = 6,
    DumbbellHammerCurlToLungeToPress = 7,
    DumbbellPushPress = 8,
    FloorInvertedShoulderPress = 9,
    WeightedFloorInvertedShoulderPress = 10,
    InvertedShoulderPress = 11,
    WeightedInvertedShoulderPress = 12,
    OneArmPushPress = 13,
    OverheadBarbellPress = 14,
    OverheadDumbbellPress = 15,
    SeatedBarbellShoulderPress = 16,
    SeatedDumbbellShoulderPress = 17,
    SingleArmDumbbellShoulderPress = 18,
    SingleArmStepUpAndPress = 19,
    SmithMachineOverheadPress = 20,
    SplitStanceHammerCurlToPress = 21,
    SwissBallDumbbellShoulderPress = 22,
    WeightPlateFrontRaise = 23,
    Unknown,
}
impl ShoulderPressExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value . 0 { 0 => Ok ( ShoulderPressExerciseName :: AlternatingDumbbellShoulderPress ) , 1 => Ok ( ShoulderPressExerciseName :: ArnoldPress ) , 2 => Ok ( ShoulderPressExerciseName :: BarbellFrontSquatToPushPress ) , 3 => Ok ( ShoulderPressExerciseName :: BarbellPushPress ) , 4 => Ok ( ShoulderPressExerciseName :: BarbellShoulderPress ) , 5 => Ok ( ShoulderPressExerciseName :: DeadCurlPress ) , 6 => Ok ( ShoulderPressExerciseName :: DumbbellAlternatingShoulderPressAndTwist ) , 7 => Ok ( ShoulderPressExerciseName :: DumbbellHammerCurlToLungeToPress ) , 8 => Ok ( ShoulderPressExerciseName :: DumbbellPushPress ) , 9 => Ok ( ShoulderPressExerciseName :: FloorInvertedShoulderPress ) , 10 => Ok ( ShoulderPressExerciseName :: WeightedFloorInvertedShoulderPress ) , 11 => Ok ( ShoulderPressExerciseName :: InvertedShoulderPress ) , 12 => Ok ( ShoulderPressExerciseName :: WeightedInvertedShoulderPress ) , 13 => Ok ( ShoulderPressExerciseName :: OneArmPushPress ) , 14 => Ok ( ShoulderPressExerciseName :: OverheadBarbellPress ) , 15 => Ok ( ShoulderPressExerciseName :: OverheadDumbbellPress ) , 16 => Ok ( ShoulderPressExerciseName :: SeatedBarbellShoulderPress ) , 17 => Ok ( ShoulderPressExerciseName :: SeatedDumbbellShoulderPress ) , 18 => Ok ( ShoulderPressExerciseName :: SingleArmDumbbellShoulderPress ) , 19 => Ok ( ShoulderPressExerciseName :: SingleArmStepUpAndPress ) , 20 => Ok ( ShoulderPressExerciseName :: SmithMachineOverheadPress ) , 21 => Ok ( ShoulderPressExerciseName :: SplitStanceHammerCurlToPress ) , 22 => Ok ( ShoulderPressExerciseName :: SwissBallDumbbellShoulderPress ) , 23 => Ok ( ShoulderPressExerciseName :: WeightPlateFrontRaise ) , _ => Ok ( ShoulderPressExerciseName :: Unknown ) , }
    }
}
#[derive(Debug)]
pub enum ShoulderStabilityExerciseName {
    NinetyDegreeCableExternalRotation = 0,
    BandExternalRotation = 1,
    BandInternalRotation = 2,
    BentArmLateralRaiseAndExternalRotation = 3,
    CableExternalRotation = 4,
    DumbbellFacePullWithExternalRotation = 5,
    FloorIRaise = 6,
    WeightedFloorIRaise = 7,
    FloorTRaise = 8,
    WeightedFloorTRaise = 9,
    FloorYRaise = 10,
    WeightedFloorYRaise = 11,
    InclineIRaise = 12,
    WeightedInclineIRaise = 13,
    InclineLRaise = 14,
    WeightedInclineLRaise = 15,
    InclineTRaise = 16,
    WeightedInclineTRaise = 17,
    InclineWRaise = 18,
    WeightedInclineWRaise = 19,
    InclineYRaise = 20,
    WeightedInclineYRaise = 21,
    LyingExternalRotation = 22,
    SeatedDumbbellExternalRotation = 23,
    StandingLRaise = 24,
    SwissBallIRaise = 25,
    WeightedSwissBallIRaise = 26,
    SwissBallTRaise = 27,
    WeightedSwissBallTRaise = 28,
    SwissBallWRaise = 29,
    WeightedSwissBallWRaise = 30,
    SwissBallYRaise = 31,
    WeightedSwissBallYRaise = 32,
    Unknown,
}
impl ShoulderStabilityExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value . 0 { 0 => Ok ( ShoulderStabilityExerciseName :: NinetyDegreeCableExternalRotation ) , 1 => Ok ( ShoulderStabilityExerciseName :: BandExternalRotation ) , 2 => Ok ( ShoulderStabilityExerciseName :: BandInternalRotation ) , 3 => Ok ( ShoulderStabilityExerciseName :: BentArmLateralRaiseAndExternalRotation ) , 4 => Ok ( ShoulderStabilityExerciseName :: CableExternalRotation ) , 5 => Ok ( ShoulderStabilityExerciseName :: DumbbellFacePullWithExternalRotation ) , 6 => Ok ( ShoulderStabilityExerciseName :: FloorIRaise ) , 7 => Ok ( ShoulderStabilityExerciseName :: WeightedFloorIRaise ) , 8 => Ok ( ShoulderStabilityExerciseName :: FloorTRaise ) , 9 => Ok ( ShoulderStabilityExerciseName :: WeightedFloorTRaise ) , 10 => Ok ( ShoulderStabilityExerciseName :: FloorYRaise ) , 11 => Ok ( ShoulderStabilityExerciseName :: WeightedFloorYRaise ) , 12 => Ok ( ShoulderStabilityExerciseName :: InclineIRaise ) , 13 => Ok ( ShoulderStabilityExerciseName :: WeightedInclineIRaise ) , 14 => Ok ( ShoulderStabilityExerciseName :: InclineLRaise ) , 15 => Ok ( ShoulderStabilityExerciseName :: WeightedInclineLRaise ) , 16 => Ok ( ShoulderStabilityExerciseName :: InclineTRaise ) , 17 => Ok ( ShoulderStabilityExerciseName :: WeightedInclineTRaise ) , 18 => Ok ( ShoulderStabilityExerciseName :: InclineWRaise ) , 19 => Ok ( ShoulderStabilityExerciseName :: WeightedInclineWRaise ) , 20 => Ok ( ShoulderStabilityExerciseName :: InclineYRaise ) , 21 => Ok ( ShoulderStabilityExerciseName :: WeightedInclineYRaise ) , 22 => Ok ( ShoulderStabilityExerciseName :: LyingExternalRotation ) , 23 => Ok ( ShoulderStabilityExerciseName :: SeatedDumbbellExternalRotation ) , 24 => Ok ( ShoulderStabilityExerciseName :: StandingLRaise ) , 25 => Ok ( ShoulderStabilityExerciseName :: SwissBallIRaise ) , 26 => Ok ( ShoulderStabilityExerciseName :: WeightedSwissBallIRaise ) , 27 => Ok ( ShoulderStabilityExerciseName :: SwissBallTRaise ) , 28 => Ok ( ShoulderStabilityExerciseName :: WeightedSwissBallTRaise ) , 29 => Ok ( ShoulderStabilityExerciseName :: SwissBallWRaise ) , 30 => Ok ( ShoulderStabilityExerciseName :: WeightedSwissBallWRaise ) , 31 => Ok ( ShoulderStabilityExerciseName :: SwissBallYRaise ) , 32 => Ok ( ShoulderStabilityExerciseName :: WeightedSwissBallYRaise ) , _ => Ok ( ShoulderStabilityExerciseName :: Unknown ) , }
    }
}
#[derive(Debug)]
pub enum ShrugExerciseName {
    BarbellJumpShrug = 0,
    BarbellShrug = 1,
    BarbellUprightRow = 2,
    BehindTheBackSmithMachineShrug = 3,
    DumbbellJumpShrug = 4,
    DumbbellShrug = 5,
    DumbbellUprightRow = 6,
    InclineDumbbellShrug = 7,
    OverheadBarbellShrug = 8,
    OverheadDumbbellShrug = 9,
    ScaptionAndShrug = 10,
    ScapularRetraction = 11,
    SerratusChairShrug = 12,
    WeightedSerratusChairShrug = 13,
    SerratusShrug = 14,
    WeightedSerratusShrug = 15,
    WideGripJumpShrug = 16,
    Unknown,
}
impl ShrugExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(ShrugExerciseName::BarbellJumpShrug),
            1 => Ok(ShrugExerciseName::BarbellShrug),
            2 => Ok(ShrugExerciseName::BarbellUprightRow),
            3 => Ok(ShrugExerciseName::BehindTheBackSmithMachineShrug),
            4 => Ok(ShrugExerciseName::DumbbellJumpShrug),
            5 => Ok(ShrugExerciseName::DumbbellShrug),
            6 => Ok(ShrugExerciseName::DumbbellUprightRow),
            7 => Ok(ShrugExerciseName::InclineDumbbellShrug),
            8 => Ok(ShrugExerciseName::OverheadBarbellShrug),
            9 => Ok(ShrugExerciseName::OverheadDumbbellShrug),
            10 => Ok(ShrugExerciseName::ScaptionAndShrug),
            11 => Ok(ShrugExerciseName::ScapularRetraction),
            12 => Ok(ShrugExerciseName::SerratusChairShrug),
            13 => Ok(ShrugExerciseName::WeightedSerratusChairShrug),
            14 => Ok(ShrugExerciseName::SerratusShrug),
            15 => Ok(ShrugExerciseName::WeightedSerratusShrug),
            16 => Ok(ShrugExerciseName::WideGripJumpShrug),
            _ => Ok(ShrugExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SitUpExerciseName {
    AlternatingSitUp = 0,
    WeightedAlternatingSitUp = 1,
    BentKneeVUp = 2,
    WeightedBentKneeVUp = 3,
    ButterflySitUp = 4,
    WeightedButterflySitup = 5,
    CrossPunchRollUp = 6,
    WeightedCrossPunchRollUp = 7,
    CrossedArmsSitUp = 8,
    WeightedCrossedArmsSitUp = 9,
    GetUpSitUp = 10,
    WeightedGetUpSitUp = 11,
    HoveringSitUp = 12,
    WeightedHoveringSitUp = 13,
    KettlebellSitUp = 14,
    MedicineBallAlternatingVUp = 15,
    MedicineBallSitUp = 16,
    MedicineBallVUp = 17,
    ModifiedSitUp = 18,
    NegativeSitUp = 19,
    OneArmFullSitUp = 20,
    RecliningCircle = 21,
    WeightedRecliningCircle = 22,
    ReverseCurlUp = 23,
    WeightedReverseCurlUp = 24,
    SingleLegSwissBallJackknife = 25,
    WeightedSingleLegSwissBallJackknife = 26,
    TheTeaser = 27,
    TheTeaserWeighted = 28,
    ThreePartRollDown = 29,
    WeightedThreePartRollDown = 30,
    VUp = 31,
    WeightedVUp = 32,
    WeightedRussianTwistOnSwissBall = 33,
    WeightedSitUp = 34,
    XAbs = 35,
    WeightedXAbs = 36,
    SitUp = 37,
    Unknown,
}
impl SitUpExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SitUpExerciseName::AlternatingSitUp),
            1 => Ok(SitUpExerciseName::WeightedAlternatingSitUp),
            2 => Ok(SitUpExerciseName::BentKneeVUp),
            3 => Ok(SitUpExerciseName::WeightedBentKneeVUp),
            4 => Ok(SitUpExerciseName::ButterflySitUp),
            5 => Ok(SitUpExerciseName::WeightedButterflySitup),
            6 => Ok(SitUpExerciseName::CrossPunchRollUp),
            7 => Ok(SitUpExerciseName::WeightedCrossPunchRollUp),
            8 => Ok(SitUpExerciseName::CrossedArmsSitUp),
            9 => Ok(SitUpExerciseName::WeightedCrossedArmsSitUp),
            10 => Ok(SitUpExerciseName::GetUpSitUp),
            11 => Ok(SitUpExerciseName::WeightedGetUpSitUp),
            12 => Ok(SitUpExerciseName::HoveringSitUp),
            13 => Ok(SitUpExerciseName::WeightedHoveringSitUp),
            14 => Ok(SitUpExerciseName::KettlebellSitUp),
            15 => Ok(SitUpExerciseName::MedicineBallAlternatingVUp),
            16 => Ok(SitUpExerciseName::MedicineBallSitUp),
            17 => Ok(SitUpExerciseName::MedicineBallVUp),
            18 => Ok(SitUpExerciseName::ModifiedSitUp),
            19 => Ok(SitUpExerciseName::NegativeSitUp),
            20 => Ok(SitUpExerciseName::OneArmFullSitUp),
            21 => Ok(SitUpExerciseName::RecliningCircle),
            22 => Ok(SitUpExerciseName::WeightedRecliningCircle),
            23 => Ok(SitUpExerciseName::ReverseCurlUp),
            24 => Ok(SitUpExerciseName::WeightedReverseCurlUp),
            25 => Ok(SitUpExerciseName::SingleLegSwissBallJackknife),
            26 => Ok(SitUpExerciseName::WeightedSingleLegSwissBallJackknife),
            27 => Ok(SitUpExerciseName::TheTeaser),
            28 => Ok(SitUpExerciseName::TheTeaserWeighted),
            29 => Ok(SitUpExerciseName::ThreePartRollDown),
            30 => Ok(SitUpExerciseName::WeightedThreePartRollDown),
            31 => Ok(SitUpExerciseName::VUp),
            32 => Ok(SitUpExerciseName::WeightedVUp),
            33 => Ok(SitUpExerciseName::WeightedRussianTwistOnSwissBall),
            34 => Ok(SitUpExerciseName::WeightedSitUp),
            35 => Ok(SitUpExerciseName::XAbs),
            36 => Ok(SitUpExerciseName::WeightedXAbs),
            37 => Ok(SitUpExerciseName::SitUp),
            _ => Ok(SitUpExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum SquatExerciseName {
    LegPress = 0,
    BackSquatWithBodyBar = 1,
    BackSquats = 2,
    WeightedBackSquats = 3,
    BalancingSquat = 4,
    WeightedBalancingSquat = 5,
    BarbellBackSquat = 6,
    BarbellBoxSquat = 7,
    BarbellFrontSquat = 8,
    BarbellHackSquat = 9,
    BarbellHangSquatSnatch = 10,
    BarbellLateralStepUp = 11,
    BarbellQuarterSquat = 12,
    BarbellSiffSquat = 13,
    BarbellSquatSnatch = 14,
    BarbellSquatWithHeelsRaised = 15,
    BarbellStepover = 16,
    BarbellStepUp = 17,
    BenchSquatWithRotationalChop = 18,
    WeightedBenchSquatWithRotationalChop = 19,
    BodyWeightWallSquat = 20,
    WeightedWallSquat = 21,
    BoxStepSquat = 22,
    WeightedBoxStepSquat = 23,
    BracedSquat = 24,
    CrossedArmBarbellFrontSquat = 25,
    CrossoverDumbbellStepUp = 26,
    DumbbellFrontSquat = 27,
    DumbbellSplitSquat = 28,
    DumbbellSquat = 29,
    DumbbellSquatClean = 30,
    DumbbellStepover = 31,
    DumbbellStepUp = 32,
    ElevatedSingleLegSquat = 33,
    WeightedElevatedSingleLegSquat = 34,
    FigureFourSquats = 35,
    WeightedFigureFourSquats = 36,
    GobletSquat = 37,
    KettlebellSquat = 38,
    KettlebellSwingOverhead = 39,
    KettlebellSwingWithFlipToSquat = 40,
    LateralDumbbellStepUp = 41,
    OneLeggedSquat = 42,
    OverheadDumbbellSquat = 43,
    OverheadSquat = 44,
    PartialSingleLegSquat = 45,
    WeightedPartialSingleLegSquat = 46,
    PistolSquat = 47,
    WeightedPistolSquat = 48,
    PlieSlides = 49,
    WeightedPlieSlides = 50,
    PlieSquat = 51,
    WeightedPlieSquat = 52,
    PrisonerSquat = 53,
    WeightedPrisonerSquat = 54,
    SingleLegBenchGetUp = 55,
    WeightedSingleLegBenchGetUp = 56,
    SingleLegBenchSquat = 57,
    WeightedSingleLegBenchSquat = 58,
    SingleLegSquatOnSwissBall = 59,
    WeightedSingleLegSquatOnSwissBall = 60,
    Squat = 61,
    WeightedSquat = 62,
    SquatsWithBand = 63,
    StaggeredSquat = 64,
    WeightedStaggeredSquat = 65,
    StepUp = 66,
    WeightedStepUp = 67,
    SuitcaseSquats = 68,
    SumoSquat = 69,
    SumoSquatSlideIn = 70,
    WeightedSumoSquatSlideIn = 71,
    SumoSquatToHighPull = 72,
    SumoSquatToStand = 73,
    WeightedSumoSquatToStand = 74,
    SumoSquatWithRotation = 75,
    WeightedSumoSquatWithRotation = 76,
    SwissBallBodyWeightWallSquat = 77,
    WeightedSwissBallWallSquat = 78,
    Thrusters = 79,
    UnevenSquat = 80,
    WeightedUnevenSquat = 81,
    WaistSlimmingSquat = 82,
    WallBall = 83,
    WideStanceBarbellSquat = 84,
    WideStanceGobletSquat = 85,
    ZercherSquat = 86,
    Unknown,
}
impl SquatExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(SquatExerciseName::LegPress),
            1 => Ok(SquatExerciseName::BackSquatWithBodyBar),
            2 => Ok(SquatExerciseName::BackSquats),
            3 => Ok(SquatExerciseName::WeightedBackSquats),
            4 => Ok(SquatExerciseName::BalancingSquat),
            5 => Ok(SquatExerciseName::WeightedBalancingSquat),
            6 => Ok(SquatExerciseName::BarbellBackSquat),
            7 => Ok(SquatExerciseName::BarbellBoxSquat),
            8 => Ok(SquatExerciseName::BarbellFrontSquat),
            9 => Ok(SquatExerciseName::BarbellHackSquat),
            10 => Ok(SquatExerciseName::BarbellHangSquatSnatch),
            11 => Ok(SquatExerciseName::BarbellLateralStepUp),
            12 => Ok(SquatExerciseName::BarbellQuarterSquat),
            13 => Ok(SquatExerciseName::BarbellSiffSquat),
            14 => Ok(SquatExerciseName::BarbellSquatSnatch),
            15 => Ok(SquatExerciseName::BarbellSquatWithHeelsRaised),
            16 => Ok(SquatExerciseName::BarbellStepover),
            17 => Ok(SquatExerciseName::BarbellStepUp),
            18 => Ok(SquatExerciseName::BenchSquatWithRotationalChop),
            19 => Ok(SquatExerciseName::WeightedBenchSquatWithRotationalChop),
            20 => Ok(SquatExerciseName::BodyWeightWallSquat),
            21 => Ok(SquatExerciseName::WeightedWallSquat),
            22 => Ok(SquatExerciseName::BoxStepSquat),
            23 => Ok(SquatExerciseName::WeightedBoxStepSquat),
            24 => Ok(SquatExerciseName::BracedSquat),
            25 => Ok(SquatExerciseName::CrossedArmBarbellFrontSquat),
            26 => Ok(SquatExerciseName::CrossoverDumbbellStepUp),
            27 => Ok(SquatExerciseName::DumbbellFrontSquat),
            28 => Ok(SquatExerciseName::DumbbellSplitSquat),
            29 => Ok(SquatExerciseName::DumbbellSquat),
            30 => Ok(SquatExerciseName::DumbbellSquatClean),
            31 => Ok(SquatExerciseName::DumbbellStepover),
            32 => Ok(SquatExerciseName::DumbbellStepUp),
            33 => Ok(SquatExerciseName::ElevatedSingleLegSquat),
            34 => Ok(SquatExerciseName::WeightedElevatedSingleLegSquat),
            35 => Ok(SquatExerciseName::FigureFourSquats),
            36 => Ok(SquatExerciseName::WeightedFigureFourSquats),
            37 => Ok(SquatExerciseName::GobletSquat),
            38 => Ok(SquatExerciseName::KettlebellSquat),
            39 => Ok(SquatExerciseName::KettlebellSwingOverhead),
            40 => Ok(SquatExerciseName::KettlebellSwingWithFlipToSquat),
            41 => Ok(SquatExerciseName::LateralDumbbellStepUp),
            42 => Ok(SquatExerciseName::OneLeggedSquat),
            43 => Ok(SquatExerciseName::OverheadDumbbellSquat),
            44 => Ok(SquatExerciseName::OverheadSquat),
            45 => Ok(SquatExerciseName::PartialSingleLegSquat),
            46 => Ok(SquatExerciseName::WeightedPartialSingleLegSquat),
            47 => Ok(SquatExerciseName::PistolSquat),
            48 => Ok(SquatExerciseName::WeightedPistolSquat),
            49 => Ok(SquatExerciseName::PlieSlides),
            50 => Ok(SquatExerciseName::WeightedPlieSlides),
            51 => Ok(SquatExerciseName::PlieSquat),
            52 => Ok(SquatExerciseName::WeightedPlieSquat),
            53 => Ok(SquatExerciseName::PrisonerSquat),
            54 => Ok(SquatExerciseName::WeightedPrisonerSquat),
            55 => Ok(SquatExerciseName::SingleLegBenchGetUp),
            56 => Ok(SquatExerciseName::WeightedSingleLegBenchGetUp),
            57 => Ok(SquatExerciseName::SingleLegBenchSquat),
            58 => Ok(SquatExerciseName::WeightedSingleLegBenchSquat),
            59 => Ok(SquatExerciseName::SingleLegSquatOnSwissBall),
            60 => Ok(SquatExerciseName::WeightedSingleLegSquatOnSwissBall),
            61 => Ok(SquatExerciseName::Squat),
            62 => Ok(SquatExerciseName::WeightedSquat),
            63 => Ok(SquatExerciseName::SquatsWithBand),
            64 => Ok(SquatExerciseName::StaggeredSquat),
            65 => Ok(SquatExerciseName::WeightedStaggeredSquat),
            66 => Ok(SquatExerciseName::StepUp),
            67 => Ok(SquatExerciseName::WeightedStepUp),
            68 => Ok(SquatExerciseName::SuitcaseSquats),
            69 => Ok(SquatExerciseName::SumoSquat),
            70 => Ok(SquatExerciseName::SumoSquatSlideIn),
            71 => Ok(SquatExerciseName::WeightedSumoSquatSlideIn),
            72 => Ok(SquatExerciseName::SumoSquatToHighPull),
            73 => Ok(SquatExerciseName::SumoSquatToStand),
            74 => Ok(SquatExerciseName::WeightedSumoSquatToStand),
            75 => Ok(SquatExerciseName::SumoSquatWithRotation),
            76 => Ok(SquatExerciseName::WeightedSumoSquatWithRotation),
            77 => Ok(SquatExerciseName::SwissBallBodyWeightWallSquat),
            78 => Ok(SquatExerciseName::WeightedSwissBallWallSquat),
            79 => Ok(SquatExerciseName::Thrusters),
            80 => Ok(SquatExerciseName::UnevenSquat),
            81 => Ok(SquatExerciseName::WeightedUnevenSquat),
            82 => Ok(SquatExerciseName::WaistSlimmingSquat),
            83 => Ok(SquatExerciseName::WallBall),
            84 => Ok(SquatExerciseName::WideStanceBarbellSquat),
            85 => Ok(SquatExerciseName::WideStanceGobletSquat),
            86 => Ok(SquatExerciseName::ZercherSquat),
            _ => Ok(SquatExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum TotalBodyExerciseName {
    Burpee = 0,
    WeightedBurpee = 1,
    BurpeeBoxJump = 2,
    WeightedBurpeeBoxJump = 3,
    HighPullBurpee = 4,
    ManMakers = 5,
    OneArmBurpee = 6,
    SquatThrusts = 7,
    WeightedSquatThrusts = 8,
    SquatPlankPushUp = 9,
    WeightedSquatPlankPushUp = 10,
    StandingTRotationBalance = 11,
    WeightedStandingTRotationBalance = 12,
    Unknown,
}
impl TotalBodyExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(TotalBodyExerciseName::Burpee),
            1 => Ok(TotalBodyExerciseName::WeightedBurpee),
            2 => Ok(TotalBodyExerciseName::BurpeeBoxJump),
            3 => Ok(TotalBodyExerciseName::WeightedBurpeeBoxJump),
            4 => Ok(TotalBodyExerciseName::HighPullBurpee),
            5 => Ok(TotalBodyExerciseName::ManMakers),
            6 => Ok(TotalBodyExerciseName::OneArmBurpee),
            7 => Ok(TotalBodyExerciseName::SquatThrusts),
            8 => Ok(TotalBodyExerciseName::WeightedSquatThrusts),
            9 => Ok(TotalBodyExerciseName::SquatPlankPushUp),
            10 => Ok(TotalBodyExerciseName::WeightedSquatPlankPushUp),
            11 => Ok(TotalBodyExerciseName::StandingTRotationBalance),
            12 => Ok(TotalBodyExerciseName::WeightedStandingTRotationBalance),
            _ => Ok(TotalBodyExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum TricepsExtensionExerciseName {
    BenchDip = 0,
    WeightedBenchDip = 1,
    BodyWeightDip = 2,
    CableKickback = 3,
    CableLyingTricepsExtension = 4,
    CableOverheadTricepsExtension = 5,
    DumbbellKickback = 6,
    DumbbellLyingTricepsExtension = 7,
    EzBarOverheadTricepsExtension = 8,
    InclineDip = 9,
    WeightedInclineDip = 10,
    InclineEzBarLyingTricepsExtension = 11,
    LyingDumbbellPulloverToExtension = 12,
    LyingEzBarTricepsExtension = 13,
    LyingTricepsExtensionToCloseGripBenchPress = 14,
    OverheadDumbbellTricepsExtension = 15,
    RecliningTricepsPress = 16,
    ReverseGripPressdown = 17,
    ReverseGripTricepsPressdown = 18,
    RopePressdown = 19,
    SeatedBarbellOverheadTricepsExtension = 20,
    SeatedDumbbellOverheadTricepsExtension = 21,
    SeatedEzBarOverheadTricepsExtension = 22,
    SeatedSingleArmOverheadDumbbellExtension = 23,
    SingleArmDumbbellOverheadTricepsExtension = 24,
    SingleDumbbellSeatedOverheadTricepsExtension = 25,
    SingleLegBenchDipAndKick = 26,
    WeightedSingleLegBenchDipAndKick = 27,
    SingleLegDip = 28,
    WeightedSingleLegDip = 29,
    StaticLyingTricepsExtension = 30,
    SuspendedDip = 31,
    WeightedSuspendedDip = 32,
    SwissBallDumbbellLyingTricepsExtension = 33,
    SwissBallEzBarLyingTricepsExtension = 34,
    SwissBallEzBarOverheadTricepsExtension = 35,
    TabletopDip = 36,
    WeightedTabletopDip = 37,
    TricepsExtensionOnFloor = 38,
    TricepsPressdown = 39,
    WeightedDip = 40,
    Unknown,
}
impl TricepsExtensionExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value . 0 { 0 => Ok ( TricepsExtensionExerciseName :: BenchDip ) , 1 => Ok ( TricepsExtensionExerciseName :: WeightedBenchDip ) , 2 => Ok ( TricepsExtensionExerciseName :: BodyWeightDip ) , 3 => Ok ( TricepsExtensionExerciseName :: CableKickback ) , 4 => Ok ( TricepsExtensionExerciseName :: CableLyingTricepsExtension ) , 5 => Ok ( TricepsExtensionExerciseName :: CableOverheadTricepsExtension ) , 6 => Ok ( TricepsExtensionExerciseName :: DumbbellKickback ) , 7 => Ok ( TricepsExtensionExerciseName :: DumbbellLyingTricepsExtension ) , 8 => Ok ( TricepsExtensionExerciseName :: EzBarOverheadTricepsExtension ) , 9 => Ok ( TricepsExtensionExerciseName :: InclineDip ) , 10 => Ok ( TricepsExtensionExerciseName :: WeightedInclineDip ) , 11 => Ok ( TricepsExtensionExerciseName :: InclineEzBarLyingTricepsExtension ) , 12 => Ok ( TricepsExtensionExerciseName :: LyingDumbbellPulloverToExtension ) , 13 => Ok ( TricepsExtensionExerciseName :: LyingEzBarTricepsExtension ) , 14 => Ok ( TricepsExtensionExerciseName :: LyingTricepsExtensionToCloseGripBenchPress ) , 15 => Ok ( TricepsExtensionExerciseName :: OverheadDumbbellTricepsExtension ) , 16 => Ok ( TricepsExtensionExerciseName :: RecliningTricepsPress ) , 17 => Ok ( TricepsExtensionExerciseName :: ReverseGripPressdown ) , 18 => Ok ( TricepsExtensionExerciseName :: ReverseGripTricepsPressdown ) , 19 => Ok ( TricepsExtensionExerciseName :: RopePressdown ) , 20 => Ok ( TricepsExtensionExerciseName :: SeatedBarbellOverheadTricepsExtension ) , 21 => Ok ( TricepsExtensionExerciseName :: SeatedDumbbellOverheadTricepsExtension ) , 22 => Ok ( TricepsExtensionExerciseName :: SeatedEzBarOverheadTricepsExtension ) , 23 => Ok ( TricepsExtensionExerciseName :: SeatedSingleArmOverheadDumbbellExtension ) , 24 => Ok ( TricepsExtensionExerciseName :: SingleArmDumbbellOverheadTricepsExtension ) , 25 => Ok ( TricepsExtensionExerciseName :: SingleDumbbellSeatedOverheadTricepsExtension ) , 26 => Ok ( TricepsExtensionExerciseName :: SingleLegBenchDipAndKick ) , 27 => Ok ( TricepsExtensionExerciseName :: WeightedSingleLegBenchDipAndKick ) , 28 => Ok ( TricepsExtensionExerciseName :: SingleLegDip ) , 29 => Ok ( TricepsExtensionExerciseName :: WeightedSingleLegDip ) , 30 => Ok ( TricepsExtensionExerciseName :: StaticLyingTricepsExtension ) , 31 => Ok ( TricepsExtensionExerciseName :: SuspendedDip ) , 32 => Ok ( TricepsExtensionExerciseName :: WeightedSuspendedDip ) , 33 => Ok ( TricepsExtensionExerciseName :: SwissBallDumbbellLyingTricepsExtension ) , 34 => Ok ( TricepsExtensionExerciseName :: SwissBallEzBarLyingTricepsExtension ) , 35 => Ok ( TricepsExtensionExerciseName :: SwissBallEzBarOverheadTricepsExtension ) , 36 => Ok ( TricepsExtensionExerciseName :: TabletopDip ) , 37 => Ok ( TricepsExtensionExerciseName :: WeightedTabletopDip ) , 38 => Ok ( TricepsExtensionExerciseName :: TricepsExtensionOnFloor ) , 39 => Ok ( TricepsExtensionExerciseName :: TricepsPressdown ) , 40 => Ok ( TricepsExtensionExerciseName :: WeightedDip ) , _ => Ok ( TricepsExtensionExerciseName :: Unknown ) , }
    }
}
#[derive(Debug)]
pub enum WarmUpExerciseName {
    QuadrupedRocking = 0,
    NeckTilts = 1,
    AnkleCircles = 2,
    AnkleDorsiflexionWithBand = 3,
    AnkleInternalRotation = 4,
    ArmCircles = 5,
    BentOverReachToSky = 6,
    CatCamel = 7,
    ElbowToFootLunge = 8,
    ForwardAndBackwardLegSwings = 9,
    Groiners = 10,
    InvertedHamstringStretch = 11,
    LateralDuckUnder = 12,
    NeckRotations = 13,
    OppositeArmAndLegBalance = 14,
    ReachRollAndLift = 15,
    Scorpion = 16,
    ShoulderCircles = 17,
    SideToSideLegSwings = 18,
    SleeperStretch = 19,
    SlideOut = 20,
    SwissBallHipCrossover = 21,
    SwissBallReachRollAndLift = 22,
    SwissBallWindshieldWipers = 23,
    ThoracicRotation = 24,
    WalkingHighKicks = 25,
    WalkingHighKnees = 26,
    WalkingKneeHugs = 27,
    WalkingLegCradles = 28,
    Walkout = 29,
    WalkoutFromPushUpPosition = 30,
    Unknown,
}
impl WarmUpExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(WarmUpExerciseName::QuadrupedRocking),
            1 => Ok(WarmUpExerciseName::NeckTilts),
            2 => Ok(WarmUpExerciseName::AnkleCircles),
            3 => Ok(WarmUpExerciseName::AnkleDorsiflexionWithBand),
            4 => Ok(WarmUpExerciseName::AnkleInternalRotation),
            5 => Ok(WarmUpExerciseName::ArmCircles),
            6 => Ok(WarmUpExerciseName::BentOverReachToSky),
            7 => Ok(WarmUpExerciseName::CatCamel),
            8 => Ok(WarmUpExerciseName::ElbowToFootLunge),
            9 => Ok(WarmUpExerciseName::ForwardAndBackwardLegSwings),
            10 => Ok(WarmUpExerciseName::Groiners),
            11 => Ok(WarmUpExerciseName::InvertedHamstringStretch),
            12 => Ok(WarmUpExerciseName::LateralDuckUnder),
            13 => Ok(WarmUpExerciseName::NeckRotations),
            14 => Ok(WarmUpExerciseName::OppositeArmAndLegBalance),
            15 => Ok(WarmUpExerciseName::ReachRollAndLift),
            16 => Ok(WarmUpExerciseName::Scorpion),
            17 => Ok(WarmUpExerciseName::ShoulderCircles),
            18 => Ok(WarmUpExerciseName::SideToSideLegSwings),
            19 => Ok(WarmUpExerciseName::SleeperStretch),
            20 => Ok(WarmUpExerciseName::SlideOut),
            21 => Ok(WarmUpExerciseName::SwissBallHipCrossover),
            22 => Ok(WarmUpExerciseName::SwissBallReachRollAndLift),
            23 => Ok(WarmUpExerciseName::SwissBallWindshieldWipers),
            24 => Ok(WarmUpExerciseName::ThoracicRotation),
            25 => Ok(WarmUpExerciseName::WalkingHighKicks),
            26 => Ok(WarmUpExerciseName::WalkingHighKnees),
            27 => Ok(WarmUpExerciseName::WalkingKneeHugs),
            28 => Ok(WarmUpExerciseName::WalkingLegCradles),
            29 => Ok(WarmUpExerciseName::Walkout),
            30 => Ok(WarmUpExerciseName::WalkoutFromPushUpPosition),
            _ => Ok(WarmUpExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum RunExerciseName {
    Run = 0,
    Walk = 1,
    Jog = 2,
    Sprint = 3,
    Unknown,
}
impl RunExerciseName {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Uint16::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(RunExerciseName::Run),
            1 => Ok(RunExerciseName::Walk),
            2 => Ok(RunExerciseName::Jog),
            3 => Ok(RunExerciseName::Sprint),
            _ => Ok(RunExerciseName::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum WaterType {
    Fresh = 0,
    Salt = 1,
    En13319 = 2,
    Custom = 3,
    Unknown,
}
impl WaterType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(WaterType::Fresh),
            1 => Ok(WaterType::Salt),
            2 => Ok(WaterType::En13319),
            3 => Ok(WaterType::Custom),
            _ => Ok(WaterType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum TissueModelType {
    #[doc = "Buhlmann\'s decompression algorithm, version C"]
    Zhl16C = 0,
    Unknown,
}
impl TissueModelType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(TissueModelType::Zhl16C),
            _ => Ok(TissueModelType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DiveGasStatus {
    Disabled = 0,
    Enabled = 1,
    BackupOnly = 2,
    Unknown,
}
impl DiveGasStatus {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DiveGasStatus::Disabled),
            1 => Ok(DiveGasStatus::Enabled),
            2 => Ok(DiveGasStatus::BackupOnly),
            _ => Ok(DiveGasStatus::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DiveAlarmType {
    Depth = 0,
    Time = 1,
    Unknown,
}
impl DiveAlarmType {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DiveAlarmType::Depth),
            1 => Ok(DiveAlarmType::Time),
            _ => Ok(DiveAlarmType::Unknown),
        }
    }
}
#[derive(Debug)]
pub enum DiveBacklightMode {
    AtDepth = 0,
    AlwaysOn = 1,
    Unknown,
}
impl DiveBacklightMode {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
        let base_value = profile::base::Enum::decode::<T>(buffer)?;
        match base_value.0 {
            0 => Ok(DiveBacklightMode::AtDepth),
            1 => Ok(DiveBacklightMode::AlwaysOn),
            _ => Ok(DiveBacklightMode::Unknown),
        }
    }
}
