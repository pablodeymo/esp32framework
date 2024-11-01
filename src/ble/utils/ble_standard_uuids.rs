/// Enums the UUIDs of standard Bluetooth Low Energy (BLE) services.
/// Each variant corresponds to a specific service defined by the Bluetooth Special Interest Group (SIG).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StandardServiceId {
    GAP = 0x1800,
    GATT = 0x1801,
    ImmediateAlert = 0x1802,
    LinkLoss = 0x1803,
    TxPower = 0x1804,
    CurrentTime = 0x1805,
    ReferenceTimeUpdate = 0x1806,
    NextDSTChange = 0x1807,
    Glucose = 0x1808,
    HealthThermometer = 0x1809,
    DeviceInformation = 0x180A,
    HeartRate = 0x180D,
    PhoneAlertStatus = 0x180E,
    Battery = 0x180F,
    BloodPressure = 0x1810,
    AlertNotification = 0x1811,
    HumanInterfaceDevice = 0x1812,
    ScanParameters = 0x1813,
    RunningSpeedAndCadence = 0x1814,
    AutomationIO = 0x1815,
    CyclingSpeedAndCadence = 0x1816,
    CyclingPower = 0x1818,
    LocationAndNavigation = 0x1819,
    EnvironmentalSensing = 0x181A,
    BodyComposition = 0x181B,
    UserData = 0x181C,
    WeightScale = 0x181D,
    BondManagement = 0x181E,
    ContinuousGlucoseMonitoring = 0x181F,
    InternetProtocolSupport = 0x1820,
    IndoorPositioning = 0x1821,
    PulseOximeter = 0x1822,
    HTTPProxy = 0x1823,
    TransportDiscovery = 0x1824,
    ObjectTransfer = 0x1825,
    FitnessMachine = 0x1826,
    MeshProvisioning = 0x1827,
    MeshProxy = 0x1828,
    ReconnectionConfiguration = 0x1829,
    InsulinDelivery = 0x183A,
    BinarySensor = 0x183B,
    EmergencyConfiguration = 0x183C,
    AuthorizationControl = 0x183D,
    PhysicalActivityMonitor = 0x183E,
    ElapsedTime = 0x183F,
    GenericHealthSensor = 0x1840,
    AudioInputControl = 0x1843,
    VolumeControl = 0x1844,
    VolumeOffsetControl = 0x1845,
    CoordinatedSetIdentification = 0x1846,
    DeviceTime = 0x1847,
    MediaControl = 0x1848,
    GenericMediaControl = 0x1849,
    ConstantToneExtension = 0x184A,
    TelephoneBearer = 0x184B,
    GenericTelephoneBearer = 0x184C,
    MicrophoneControl = 0x184D,
    AudioStreamControl = 0x184E,
    BroadcastAudioScan = 0x184F,
    PublishedAudioCapabilities = 0x1850,
    BasicAudioAnnouncement = 0x1851,
    BroadcastAudioAnnouncement = 0x1852,
    CommonAudio = 0x1853,
    HearingAccess = 0x1854,
    TelephonyAndMediaAudio = 0x1855,
    PublicBroadcastAnnouncement = 0x1856,
    ElectronicShelfLabel = 0x1857,
    GamingAudio = 0x1858,
    MeshProxySolicitation = 0x1859,
}

impl StandardServiceId {
    /// Byte size of the StandarServiceId
    ///
    /// # Returns
    ///
    /// An usize representing the size
    pub fn byte_size(&self) -> usize {
        2
    }
}

/// Enums the UUIDs of standard Bluetooth Low Energy (BLE) characteristics.
/// Each variant corresponds to a specific characteristic defined by the Bluetooth Special Interest Group (SIG).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StandardCharacteristicId {
    DeviceName = 10752,
    Appearance = 10753,
    PeripheralPrivacyFlag = 10754,
    ReconnectionAddress = 10755,
    PeripheralPreferredConnectionParameters = 10756,
    ServiceChanged = 10757,
    AlertLevel = 10758,
    TxPowerLevel = 10759,
    DateTime = 10760,
    DayofWeek = 10761,
    DayDateTime = 10762,
    ExactTime256 = 10764,
    DSTOffset = 10765,
    TimeZone = 10766,
    LocalTimeInformation = 10767,
    TimewithDST = 10769,
    TimeAccuracy = 10770,
    TimeSource = 10771,
    ReferenceTimeInformation = 10772,
    TimeUpdateControlPoint = 10774,
    TimeUpdateState = 10775,
    GlucoseMeasurement = 10776,
    BatteryLevel = 10777,
    TemperatureMeasurement = 10780,
    TemperatureType = 10781,
    IntermediateTemperature = 10782,
    MeasurementInterval = 10785,
    BootKeyboardInputReport = 10786,
    SystemID = 10787,
    ModelNumberString = 10788,
    SerialNumberString = 10789,
    FirmwareRevisionString = 10790,
    HardwareRevisionString = 10791,
    SoftwareRevisionString = 10792,
    ManufacturerNameString = 10793,
    IEEE1107320601RegulatoryCertificationDataList = 10794,
    CurrentTime = 10795,
    MagneticDeclination = 10796,
    ScanRefresh = 10801,
    BootKeyboardOutputReport = 10802,
    BootMouseInputReport = 10803,
    GlucoseMeasurementContext = 10804,
    BloodPressureMeasurement = 10805,
    IntermediateCuffPressure = 10806,
    HeartRateMeasurement = 10807,
    BodySensorLocation = 10808,
    HeartRateControlPoint = 10809,
    AlertStatus = 10815,
    RingerControlPoint = 10816,
    RingerSetting = 10817,
    AlertCategoryIDBitMask = 10818,
    AlertCategoryID = 10819,
    AlertNotificationControlPoint = 10820,
    UnreadAlertStatus = 10821,
    NewAlert = 10822,
    SupportedNewAlertCategory = 10823,
    SupportedUnreadAlertCategory = 10824,
    BloodPressureFeature = 10825,
    HIDInformation = 10826,
    ReportMap = 10827,
    HIDControlPoint = 10828,
    Report = 10829,
    ProtocolMode = 10830,
    ScanIntervalWindow = 10831,
    PnPID = 10832,
    GlucoseFeature = 10833,
    RecordAccessControlPoint = 10834,
    RSCMeasurement = 10835,
    RSCFeature = 10836,
    SCControlPoint = 10837,
    Aggregate = 10842,
    CSCMeasurement = 10843,
    CSCFeature = 10844,
    SensorLocation = 10845,
    PLXSpotCheckMeasurement = 10846,
    PLXContinuousMeasurement = 10847,
    PLXFeatures = 10848,
    CyclingPowerMeasurement = 10851,
    CyclingPowerVector = 10852,
    CyclingPowerFeature = 10853,
    CyclingPowerControlPoint = 10854,
    LocationandSpeed = 10855,
    Navigation = 10856,
    PositionQuality = 10857,
    LNFeature = 10858,
    LNControlPoint = 10859,
    Elevation = 10860,
    Pressure = 10861,
    Temperature = 10862,
    Humidity = 10863,
    TrueWindSpeed = 10864,
    TrueWindDirection = 10865,
    ApparentWindSpeed = 10866,
    ApparentWindDirection = 10867,
    GustFactor = 10868,
    PollenConcentration = 10869,
    UVIndex = 10870,
    Irradiance = 10871,
    Rainfall = 10872,
    WindChill = 10873,
    HeatIndex = 10874,
    DewPoint = 10875,
    DescriptorValueChanged = 10877,
    AerobicHeartRateLowerLimit = 10878,
    AerobicThreshold = 10879,
    Age = 10880,
    AnaerobicHeartRateLowerLimit = 10881,
    AnaerobicHeartRateUpperLimit = 10882,
    AnaerobicThreshold = 10883,
    AerobicHeartRateUpperLimit = 10884,
    DateofBirth = 10885,
    DateofThresholdAssessment = 10886,
    EmailAddress = 10887,
    FatBurnHeartRateLowerLimit = 10888,
    FatBurnHeartRateUpperLimit = 10889,
    FirstName = 10890,
    FiveZoneHeartRateLimits = 10891,
    Gender = 10892,
    HeartRateMax = 10893,
    Height = 10894,
    HipCircumference = 10895,
    LastName = 10896,
    MaximumRecommendedHeartRate = 10897,
    RestingHeartRate = 10898,
    SportTypeforAerobicandAnaerobicThresholds = 10899,
    ThreeZoneHeartRateLimits = 10900,
    TwoZoneHeartRateLimits = 10901,
    VO2Max = 10902,
    WaistCircumference = 10903,
    Weight = 10904,
    DatabaseChangeIncrement = 10905,
    UserIndex = 10906,
    BodyCompositionFeature = 10907,
    BodyCompositionMeasurement = 10908,
    WeightMeasurement = 10909,
    WeightScaleFeature = 10910,
    UserControlPoint = 10911,
    MagneticFluxDensity2D = 10912,
    MagneticFluxDensity3D = 10913,
    Language = 10914,
    BarometricPressureTrend = 10915,
    BondManagementControlPoint = 10916,
    BondManagementFeature = 10917,
    CentralAddressResolution = 10918,
    CGMMeasurement = 10919,
    CGMFeature = 10920,
    CGMStatus = 10921,
    CGMSessionStartTime = 10922,
    CGMSessionRunTime = 10923,
    CGMSpecificOpsControlPoint = 10924,
    IndoorPositioningConfiguration = 10925,
    Latitude = 10926,
    Longitude = 10927,
    LocalNorthCoordinate = 10928,
    LocalEastCoordinate = 10929,
    FloorNumber = 10930,
    Altitude = 10931,
    Uncertainty = 10932,
    LocationName = 10933,
    URI = 10934,
    HTTPHeaders = 10935,
    HTTPStatusCode = 10936,
    HTTPEntityBody = 10937,
    HTTPControlPoint = 10938,
    HTTPSSecurity = 10939,
    TDSControlPoint = 10940,
    OTSFeature = 10941,
    ObjectName = 10942,
    ObjectType = 10943,
    ObjectSize = 10944,
    ObjectFirstCreated = 10945,
    ObjectLastModified = 10946,
    ObjectID = 10947,
    ObjectProperties = 10948,
    ObjectActionControlPoint = 10949,
    ObjectListControlPoint = 10950,
    ObjectListFilter = 10951,
    ObjectChanged = 10952,
    ResolvablePrivateAddressOnly = 10953,
    FitnessMachineFeature = 10956,
    TreadmillData = 10957,
    CrossTrainerData = 10958,
    StepClimberData = 10959,
    StairClimberData = 10960,
    RowerData = 10961,
    IndoorBikeData = 10962,
    TrainingStatus = 10963,
    SupportedSpeedRange = 10964,
    SupportedInclinationRange = 10965,
    SupportedResistanceLevelRange = 10966,
    SupportedHeartRateRange = 10967,
    SupportedPowerRange = 10968,
    FitnessMachineControlPoint = 10969,
    FitnessMachineStatus = 10970,
    MeshProvisioningDataIn = 10971,
    MeshProvisioningDataOut = 10972,
    MeshProxyDataIn = 10973,
    MeshProxyDataOut = 10974,
    AverageCurrent = 10976,
    AverageVoltage = 10977,
    Boolean = 10978,
    ChromaticDistancefromPlanckian = 10979,
    ChromaticityCoordinates = 10980,
    ChromaticityinCCTandDuvValues = 10981,
    ChromaticityTolerance = 10982,
    CIE1331995ColorRenderingIndex = 10983,
    Coefficient = 10984,
    CorrelatedColorTemperature = 10985,
    Count16 = 10986,
    Count24 = 10987,
    CountryCode = 10988,
    DateUTC = 10989,
    ElectricCurrent = 10990,
    ElectricCurrentRange = 10991,
    ElectricCurrentSpecification = 10992,
    ElectricCurrentStatistics = 10993,
    Energy = 10994,
    EnergyinaPeriodofDay = 10995,
    EventStatistics = 10996,
    FixedString16 = 10997,
    FixedString24 = 10998,
    FixedString36 = 10999,
    FixedString8 = 11000,
    GenericLevel = 11001,
    GlobalTradeItemNumber = 11002,
    Illuminance = 11003,
    LuminousEfficacy = 11004,
    LuminousEnergy = 11005,
    LuminousExposure = 11006,
    LuminousFlux = 11007,
    LuminousFluxRange = 11008,
    LuminousIntensity = 11009,
    MassFlow = 11010,
    PerceivedLightness = 11011,
    Percentage8 = 11012,
    Power = 11013,
    PowerSpecification = 11014,
    RelativeRuntimeinaCurrentRange = 11015,
    RelativeRuntimeinaGenericLevelRange = 11016,
    RelativeValueinaVoltageRange = 11017,
    RelativeValueinanIlluminanceRange = 11018,
    RelativeValueinaPeriodofDay = 11019,
    RelativeValueinaTemperatureRange = 11020,
    Temperature8 = 11021,
    Temperature8inaPeriodofDay = 11022,
    Temperature8Statistics = 11023,
    TemperatureRange = 11024,
    TemperatureStatistics = 11025,
    TimeDecihour8 = 11026,
    TimeExponential8 = 11027,
    TimeHour24 = 11028,
    TimeMillisecond24 = 11029,
    TimeSecond16 = 11030,
    TimeSecond8 = 11031,
    Voltage = 11032,
    VoltageSpecification = 11033,
    VoltageStatistics = 11034,
    VolumeFlow = 11035,
    ChromaticityCoordinate = 11036,
    RCFeature = 11037,
    RCSettings = 11038,
    ReconnectionConfigurationControlPoint = 11039,
    IDDStatusChanged = 11040,
    IDDStatus = 11041,
    IDDAnnunciationStatus = 11042,
    IDDFeatures = 11043,
    IDDStatusReaderControlPoint = 11044,
    IDDCommandControlPoint = 11045,
    IDDCommandData = 11046,
    IDDRecordAccessControlPoint = 11047,
    IDDHistoryData = 11048,
    ClientSupportedFeatures = 11049,
    DatabaseHash = 11050,
    BSSControlPoint = 11051,
    BSSResponse = 11052,
    EmergencyID = 11053,
    EmergencyText = 11054,
    ACSStatus = 11055,
    ACSDataIn = 11056,
    ACSDataOutNotify = 11057,
    ACSDataOutIndicate = 11058,
    ACSControlPoint = 11059,
    EnhancedBloodPressureMeasurement = 11060,
    EnhancedIntermediateCuffPressure = 11061,
    BloodPressureRecord = 11062,
    RegisteredUser = 11063,
    BREDRHandoverData = 11064,
    BluetoothSIGData = 11065,
    ServerSupportedFeatures = 11066,
    PhysicalActivityMonitorFeatures = 11067,
    GeneralActivityInstantaneousData = 11068,
    GeneralActivitySummaryData = 11069,
    CardioRespiratoryActivityInstantaneousData = 11070,
    CardioRespiratoryActivitySummaryData = 11071,
    StepCounterActivitySummaryData = 11072,
    SleepActivityInstantaneousData = 11073,
    SleepActivitySummaryData = 11074,
    PhysicalActivityMonitorControlPoint = 11075,
    PhysicalActivityCurrentSession = 11076,
    PhysicalActivitySessionDescriptor = 11077,
    PreferredUnits = 11078,
    HighResolutionHeight = 11079,
    MiddleName = 11080,
    StrideLength = 11081,
    Handedness = 11082,
    DeviceWearingPosition = 11083,
    FourZoneHeartRateLimits = 11084,
    HighIntensityExerciseThreshold = 11085,
    ActivityGoal = 11086,
    SedentaryIntervalNotification = 11087,
    CaloricIntake = 11088,
    TMAPRole = 11089,
    AudioInputState = 11127,
    GainSettingsAttribute = 11128,
    AudioInputType = 11129,
    AudioInputStatus = 11130,
    AudioInputControlPoint = 11131,
    AudioInputDescription = 11132,
    VolumeState = 11133,
    VolumeControlPoint = 11134,
    VolumeFlags = 11135,
    VolumeOffsetState = 11136,
    AudioLocation = 11137,
    VolumeOffsetControlPoint = 11138,
    AudioOutputDescription = 11139,
    SetIdentityResolvingKey = 11140,
    CoordinatedSetSize = 11141,
    SetMemberLock = 11142,
    SetMemberRank = 11143,
    EncryptedDataKeyMaterial = 11144,
    ApparentEnergy32 = 11145,
    ApparentPower = 11146,
    LiveHealthObservations = 11147,
    COtextsubscript2Concentration = 11148,
    CosineoftheAngle = 11149,
    DeviceTimeFeature = 11150,
    DeviceTimeParameters = 11151,
    DeviceTime = 11152,
    DeviceTimeControlPoint = 11153,
    TimeChangeLogData = 11154,
    MediaPlayerName = 11155,
    MediaPlayerIconObjectID = 11156,
    MediaPlayerIconURL = 11157,
    TrackChanged = 11158,
    TrackTitle = 11159,
    TrackDuration = 11160,
    TrackPosition = 11161,
    PlaybackSpeed = 11162,
    SeekingSpeed = 11163,
    CurrentTrackSegmentsObjectID = 11164,
    CurrentTrackObjectID = 11165,
    NextTrackObjectID = 11166,
    ParentGroupObjectID = 11167,
    CurrentGroupObjectID = 11168,
    PlayingOrder = 11169,
    PlayingOrdersSupported = 11170,
    MediaState = 11171,
    MediaControlPoint = 11172,
    MediaControlPointOpcodesSupported = 11173,
    SearchResultsObjectID = 11174,
    SearchControlPoint = 11175,
    Energy32 = 11176,
    ConstantToneExtensionEnable = 11181,
    AdvertisingConstantToneExtensionMinimumLength = 11182,
    AdvertisingConstantToneExtensionMinimumTransmitCount = 11183,
    AdvertisingConstantToneExtensionTransmitDuration = 11184,
    AdvertisingConstantToneExtensionInterval = 11185,
    AdvertisingConstantToneExtensionPHY = 11186,
    BearerProviderName = 11187,
    BearerUCI = 11188,
    BearerTechnology = 11189,
    BearerURISchemesSupportedList = 11190,
    BearerSignalStrength = 11191,
    BearerSignalStrengthReportingInterval = 11192,
    BearerListCurrentCalls = 11193,
    ContentControlID = 11194,
    StatusFlags = 11195,
    IncomingCallTargetBearerURI = 11196,
    CallState = 11197,
    CallControlPoint = 11198,
    CallControlPointOptionalOpcodes = 11199,
    TerminationReason = 11200,
    IncomingCall = 11201,
    CallFriendlyName = 11202,
    Mute = 11203,
    SinkASE = 11204,
    SourceASE = 11205,
    ASEControlPoint = 11206,
    BroadcastAudioScanControlPoint = 11207,
    BroadcastReceiveState = 11208,
    SinkPAC = 11209,
    SinkAudioLocations = 11210,
    SourcePAC = 11211,
    SourceAudioLocations = 11212,
    AvailableAudioContexts = 11213,
    SupportedAudioContexts = 11214,
    AmmoniaConcentration = 11215,
    CarbonMonoxideConcentration = 11216,
    MethaneConcentration = 11217,
    NitrogenDioxideConcentration = 11218,
    NonMethaneVolatileOrganicCompoundsConcentration = 11219,
    OzoneConcentration = 11220,
    ParticulateMatterPM1Concentration = 11221,
    ParticulateMatterPM25Concentration = 11222,
    ParticulateMatterPM10Concentration = 11223,
    SulfurDioxideConcentration = 11224,
    SulfurHexafluorideConcentration = 11225,
    HearingAidFeatures = 11226,
    HearingAidPresetControlPoint = 11227,
    ActivePresetIndex = 11228,
    StoredHealthObservations = 11229,
    FixedString64 = 11230,
    HighTemperature = 11231,
    HighVoltage = 11232,
    LightDistribution = 11233,
    LightOutput = 11234,
    LightSourceType = 11235,
    Noise = 11236,
    RelativeRuntimeinaCorrelatedColorTemperatureRange = 11237,
    TimeSecond32 = 11238,
    VOCConcentration = 11239,
    VoltageFrequency = 11240,
    BatteryCriticalStatus = 11241,
    BatteryHealthStatus = 11242,
    BatteryHealthInformation = 11243,
    BatteryInformation = 11244,
    BatteryLevelStatus = 11245,
    BatteryTimeStatus = 11246,
    EstimatedServiceDate = 11247,
    BatteryEnergyStatus = 11248,
    ObservationScheduleChanged = 11249,
    CurrentElapsedTime = 11250,
    HealthSensorFeatures = 11251,
    GHSControlPoint = 11252,
    LEGATTSecurityLevels = 11253,
    ESLAddress = 11254,
    APSyncKeyMaterial = 11255,
    ESLResponseKeyMaterial = 11256,
    ESLCurrentAbsoluteTime = 11257,
    ESLDisplayInformation = 11258,
    ESLImageInformation = 11259,
    ESLSensorInformation = 11260,
    ESLLEDInformation = 11261,
    ESLControlPoint = 11262,
    UDIforMedicalDevices = 11263,
    GMAPRole = 11264,
    UGGFeatures = 11265,
    UGTFeatures = 11266,
    BGSFeatures = 11267,
    BGRFeatures = 11268,
    Percentage8Steps = 11269,
}

impl StandardCharacteristicId {
    /// Byte size of the StandarCharacteristicId
    ///
    /// # Returns
    ///
    /// An usize representing the size
    pub fn byte_size(&self) -> usize {
        2
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StandardDescriptorId {
    CharacteristicExtendedProperties = 10496,
    CharacteristicUserDescription = 10497,
    ClientCharacteristicConfiguration = 10498,
    ServerCharacteristicConfiguration = 10499,
    CharacteristicPresentationFormat = 10500,
    CharacteristicAggregateFormat = 10501,
    ValidRange = 10502,
    ExternalReportReference = 10503,
    ReportReference = 10504,
    NumberofDigitals = 10505,
    ValueTriggerSetting = 10506,
    EnvironmentalSensingConfiguration = 10507,
    EnvironmentalSensingMeasurement = 10508,
    EnvironmentalSensingTriggerSetting = 10509,
    TimeTriggerSetting = 10510,
    CompleteBREDRTransportBlockData = 10511,
    ObservationSchedule = 10512,
    ValidRangeandAccuracy = 10513,
}

impl StandardDescriptorId {
    pub fn byte_size(&self) -> usize {
        2
    }
}
