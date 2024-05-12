use phf::phf_map;
use uncased::UncasedStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum KeywordKind {
    AudioChannels,
    AudioCodec,
    AudioLanguage,
    DeviceCompatibility,
    Episode,
    EpisodeType,
    FileExtension,
    Language,
    Other,
    ReleaseGroup,
    ReleaseInformation,
    ReleaseVersion,
    Season,
    Source,
    Subtitles,
    Type,
    VideoCodec,
    VideoColorDepth,
    VideoFormat,
    VideoFrameRate,
    VideoProfile,
    VideoQuality,
    VideoResolution,
    Volume,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Keyword {
    pub(crate) kind: KeywordKind,
    flags: u8,
}

impl Keyword {
    const AMBIGUOUS: u8 = 1 << 0;
    const UNBOUNDED: u8 = 1 << 1;

    pub(crate) const fn new(kind: KeywordKind) -> Self {
        Self { kind, flags: 0 }
    }

    pub(crate) const fn unbounded(kind: KeywordKind) -> Self {
        Self {
            kind,
            flags: Self::UNBOUNDED,
        }
    }

    pub(crate) const fn ambiguous(kind: KeywordKind) -> Self {
        Self {
            kind,
            flags: Self::AMBIGUOUS,
        }
    }

    pub(crate) const fn is_ambiguous(&self) -> bool {
        (self.flags & Self::AMBIGUOUS) == Self::AMBIGUOUS
    }

    pub(crate) const fn is_bounded(&self) -> bool {
        (self.flags & Self::UNBOUNDED) != Self::UNBOUNDED
    }
}

pub(crate) static KEYWORDS: phf::Map<&'static UncasedStr, Keyword> = phf_map! {
    // Audio
    //
    // Channels
    UncasedStr::new("2.0ch")        =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("2ch")          =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("5.1")          =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("5.1ch")        =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("7.1")          =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("7.1ch")        =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("DTS")          =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("DTS-ES")       =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("DTS5.1")       =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("Dolby TrueHD") =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("TrueHD")       =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("TrueHD5.1")    =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("DD5.1")        =>    Keyword::new(KeywordKind::AudioChannels),
    UncasedStr::new("DD2.0")        =>    Keyword::new(KeywordKind::AudioChannels),
    // Codec
    UncasedStr::new("AAC")          =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("AAC2.0")       =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("AACX2")        =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("AACX3")        =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("AACX4")        =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("AC3")          =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("EAC3")         =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("E-AC-3")       =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("FLAC")         =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("FLACX2")       =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("FLACX3")       =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("FLACX4")       =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("Lossless")     =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("MP3")          =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("OGG")          =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("Vorbis")       =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("Atmos")        =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("Dolby Atmos")  =>    Keyword::new(KeywordKind::AudioCodec),
    UncasedStr::new("Opus")         =>    Keyword::ambiguous(KeywordKind::AudioCodec),  // e.g. "Opus.COLORs"
    // Language
    UncasedStr::new("DualAudio")    =>    Keyword::new(KeywordKind::AudioLanguage),
    UncasedStr::new("Dual Audio")   =>    Keyword::new(KeywordKind::AudioLanguage),
    UncasedStr::new("Dual-Audio")   =>    Keyword::new(KeywordKind::AudioLanguage),

    // Device compatibility
    UncasedStr::new("Android")      =>    Keyword::ambiguous(KeywordKind::DeviceCompatibility),  // e.g. "Dragon Ball Z: Super Android 13"
    UncasedStr::new("iPad3")        =>    Keyword::new(KeywordKind::DeviceCompatibility),
    UncasedStr::new("iPhone5")      =>    Keyword::new(KeywordKind::DeviceCompatibility),
    UncasedStr::new("iPod")         =>    Keyword::new(KeywordKind::DeviceCompatibility),
    UncasedStr::new("PS3")          =>    Keyword::new(KeywordKind::DeviceCompatibility),
    UncasedStr::new("Xbox")         =>    Keyword::new(KeywordKind::DeviceCompatibility),
    UncasedStr::new("Xbox360")      =>    Keyword::new(KeywordKind::DeviceCompatibility),

    // Episode prefix
    UncasedStr::new("Ep")           =>    Keyword::new(KeywordKind::Episode),
    UncasedStr::new("Eps")          =>    Keyword::new(KeywordKind::Episode),
    UncasedStr::new("Episode")      =>    Keyword::new(KeywordKind::Episode),
    UncasedStr::new("Episodes")     =>    Keyword::new(KeywordKind::Episode),
    UncasedStr::new("Episodio")     =>    Keyword::new(KeywordKind::Episode),
    UncasedStr::new("Episódio")     =>    Keyword::new(KeywordKind::Episode),
    UncasedStr::new("Capitulo")     =>    Keyword::new(KeywordKind::Episode),
    UncasedStr::new("Folge")        =>    Keyword::new(KeywordKind::Episode),

    // Episode type
    UncasedStr::new("OP")           =>    Keyword::ambiguous(KeywordKind::EpisodeType),
    UncasedStr::new("Opening")      =>    Keyword::ambiguous(KeywordKind::EpisodeType),
    UncasedStr::new("ED")           =>    Keyword::ambiguous(KeywordKind::EpisodeType),
    UncasedStr::new("Ending")       =>    Keyword::ambiguous(KeywordKind::EpisodeType),
    UncasedStr::new("NCED")         =>    Keyword::new(KeywordKind::EpisodeType),
    UncasedStr::new("NCOP")         =>    Keyword::new(KeywordKind::EpisodeType),
    UncasedStr::new("Preview")      =>    Keyword::ambiguous(KeywordKind::EpisodeType),
    UncasedStr::new("PV")           =>    Keyword::ambiguous(KeywordKind::EpisodeType),

    // File extension
    UncasedStr::new("3gp")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("avi")          =>    Keyword::new(KeywordKind::FileExtension),
    // UncasedStr::new("divx")         =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("flv")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("m2ts")         =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("mkv")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("mov")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("mp4")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("mpg")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("ogm")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("rm")           =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("rmvb")         =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("ts")           =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("webm")         =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("wmv")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("ass")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("srt")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("ssa")          =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("7z")           =>    Keyword::new(KeywordKind::FileExtension),
    UncasedStr::new("zip")          =>    Keyword::new(KeywordKind::FileExtension),

    // Language
    UncasedStr::new("ENG")          =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("English")      =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("ESP")          =>    Keyword::ambiguous(KeywordKind::Language),  // e.g. "Tokyo ESP"
    UncasedStr::new("Espanol")      =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("Spanish")      =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("ITA")          =>    Keyword::ambiguous(KeywordKind::Language),  // e.g. "Bokura ga Ita"
    UncasedStr::new("JAP")          =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("JP")           =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("JA")           =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("JPN")          =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("PT-BR")        =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("VOSTFR")       =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("CHT")          =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("CHS")          =>    Keyword::new(KeywordKind::Language),
    UncasedStr::new("CHI")          =>    Keyword::new(KeywordKind::Language),

    // Other
    UncasedStr::new("Remaster")     =>    Keyword::new(KeywordKind::Other),
    UncasedStr::new("Remastered")   =>    Keyword::new(KeywordKind::Other),
    UncasedStr::new("Uncensored")   =>    Keyword::new(KeywordKind::Other),
    UncasedStr::new("Uncut")        =>    Keyword::new(KeywordKind::Other),
    // UncasedStr::new("TS")           =>    Keyword::new(KeywordKind::Other),
    UncasedStr::new("VFR")          =>    Keyword::new(KeywordKind::Other),
    UncasedStr::new("Widescreen")   =>    Keyword::new(KeywordKind::Other),
    UncasedStr::new("WS")           =>    Keyword::new(KeywordKind::Other),

    // Release group
    UncasedStr::new("THORA")        =>    Keyword::new(KeywordKind::ReleaseGroup),  // special case because usually placed at the end
    UncasedStr::new("UTW-THORA")    =>    Keyword::new(KeywordKind::ReleaseGroup),  // due to special case above, parser can't handle compound ones
    UncasedStr::new("JPTVclub")     =>    Keyword::new(KeywordKind::ReleaseGroup),  // usually at the end

    // Release information
    UncasedStr::new("Batch")        =>    Keyword::new(KeywordKind::ReleaseInformation),
    UncasedStr::new("Complete")     =>    Keyword::new(KeywordKind::ReleaseInformation),
    UncasedStr::new("End")          =>    Keyword::ambiguous(KeywordKind::ReleaseInformation),  // e.g. "The End of Evangelion"
    UncasedStr::new("Final")        =>    Keyword::ambiguous(KeywordKind::ReleaseInformation),  // e.g. "Final Approach"
    UncasedStr::new("Patch")        =>    Keyword::new(KeywordKind::ReleaseInformation),
    UncasedStr::new("Remux")        =>    Keyword::new(KeywordKind::ReleaseInformation),

    // Release version
    UncasedStr::new("v0")           =>    Keyword::new(KeywordKind::ReleaseVersion),
    UncasedStr::new("v1")           =>    Keyword::new(KeywordKind::ReleaseVersion),
    UncasedStr::new("v2")           =>    Keyword::new(KeywordKind::ReleaseVersion),
    UncasedStr::new("v3")           =>    Keyword::new(KeywordKind::ReleaseVersion),
    UncasedStr::new("v4")           =>    Keyword::new(KeywordKind::ReleaseVersion),

    // Season
    // Usually preceded or followed by a number (e.g. `2nd Season` or `Season 2`).
    UncasedStr::new("Season")       =>    Keyword::ambiguous(KeywordKind::Season),
    UncasedStr::new("Saison")       =>    Keyword::ambiguous(KeywordKind::Season),

    // Source
    //
    // Blu-ray
    UncasedStr::new("BD")           =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("BDRip")        =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("BluRay")       =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("Blu-ray")      =>    Keyword::new(KeywordKind::Source),
    // DVD
    UncasedStr::new("DVD")          =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("DVD5")         =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("DVD9")         =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("DVDISO")       =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("DVDRip")       =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("DVD-Rip")      =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("R2DVD")        =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("R2J")          =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("R2JDVD")       =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("R2JDVDRip")    =>    Keyword::new(KeywordKind::Source),
    // TV
    UncasedStr::new("HDTV")         =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("HDTVRip")      =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("TVRip")        =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("TV-Rip")       =>    Keyword::new(KeywordKind::Source),
    // Web
    UncasedStr::new("Web")          =>    Keyword::ambiguous(KeywordKind::Source),
    UncasedStr::new("Webcast")      =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("WebDL")        =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("Web-DL")       =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("WebRip")       =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("AMZN")         =>    Keyword::new(KeywordKind::Source),  // Amazon Prime
    UncasedStr::new("CR")           =>    Keyword::new(KeywordKind::Source),  // Crunchyroll
    UncasedStr::new("Crunchyroll")  =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("DSNP")         =>    Keyword::new(KeywordKind::Source),  // Disney+
    UncasedStr::new("Funi")         =>    Keyword::new(KeywordKind::Source),  // Funimation
    UncasedStr::new("Funimation")   =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("HIDI")         =>    Keyword::new(KeywordKind::Source),  // Hidive
    UncasedStr::new("Hidive")       =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("Hulu")         =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("Netflix")      =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("NF")           =>    Keyword::new(KeywordKind::Source),  // Netflix
    UncasedStr::new("VRV")          =>    Keyword::new(KeywordKind::Source),
    UncasedStr::new("YouTube")      =>    Keyword::new(KeywordKind::Source),

    // Subtitles
    // UncasedStr::new("ASS")          =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("BIG5")         =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Dub")          =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Dubbed")       =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Hardsub")      =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Hardsubs")     =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("RAW")          =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Softsub")      =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Softsubs")     =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Sub")          =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Subbed")       =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Subtitled")    =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Multisub")     =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Multi Sub")    =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("Multi-Sub")    =>    Keyword::new(KeywordKind::Subtitles),
    UncasedStr::new("CC")           =>    Keyword::ambiguous(KeywordKind::Subtitles),
    UncasedStr::new("SDH")          =>    Keyword::ambiguous(KeywordKind::Subtitles),

    // Type
    UncasedStr::new("TV")           =>    Keyword::ambiguous(KeywordKind::Type),
    UncasedStr::new("Movie")        =>    Keyword::ambiguous(KeywordKind::Type),
    UncasedStr::new("Gekijouban")   =>    Keyword::ambiguous(KeywordKind::Type),
    UncasedStr::new("OAD")          =>    Keyword::ambiguous(KeywordKind::Type),
    UncasedStr::new("OAV")          =>    Keyword::ambiguous(KeywordKind::Type),
    UncasedStr::new("ONA")          =>    Keyword::ambiguous(KeywordKind::Type),
    UncasedStr::new("OVA")          =>    Keyword::ambiguous(KeywordKind::Type),
    UncasedStr::new("SP")           =>    Keyword::ambiguous(KeywordKind::Type),  // e.g. "Yumeiro Patissiere SP Professional"
    UncasedStr::new("Special")      =>    Keyword::ambiguous(KeywordKind::Type),
    UncasedStr::new("Specials")     =>    Keyword::ambiguous(KeywordKind::Type),

    // Video
    //
    // Color depth
    UncasedStr::new("8bit")         =>    Keyword::new(KeywordKind::VideoColorDepth),
    UncasedStr::new("8-bit")        =>    Keyword::new(KeywordKind::VideoColorDepth),
    UncasedStr::new("10bit")        =>    Keyword::new(KeywordKind::VideoColorDepth),
    UncasedStr::new("10bits")       =>    Keyword::new(KeywordKind::VideoColorDepth),
    UncasedStr::new("10-bit")       =>    Keyword::new(KeywordKind::VideoColorDepth),
    UncasedStr::new("10-bits")      =>    Keyword::new(KeywordKind::VideoColorDepth),
    // Codec
    UncasedStr::new("AV1")          =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("AVC")          =>    Keyword::new(KeywordKind::VideoCodec),
    // UncasedStr::new("DivX")         =>    Keyword::new(KeywordKind::VideoCodec),  // @Warning: Duplicate
    UncasedStr::new("DivX5")        =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("DivX6")        =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("H.264")        =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("H.265")        =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("X.264")        =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("H264")         =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("H265")         =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("X264")         =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("X265")         =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("HEVC")         =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("HEVC2")        =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("Xvid")         =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("HDR")          =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("DV")           =>    Keyword::new(KeywordKind::VideoCodec),
    UncasedStr::new("Dolby Vision") =>    Keyword::new(KeywordKind::VideoCodec),
    // Format
    // UncasedStr::new("AVI")          =>    Keyword::new(KeywordKind::VideoFormat),  // @Warning: Duplicate
    // UncasedStr::new("RMVB")         =>    Keyword::new(KeywordKind::VideoFormat),  // @Warning: Duplicate
    // UncasedStr::new("WMV")          =>    Keyword::new(KeywordKind::VideoFormat),  // @Warning: Duplicate
    UncasedStr::new("WMV3")         =>    Keyword::new(KeywordKind::VideoFormat),
    UncasedStr::new("WMV9")         =>    Keyword::new(KeywordKind::VideoFormat),
    // Frame rate
    UncasedStr::new("23.976FPS")    =>    Keyword::new(KeywordKind::VideoFrameRate),
    UncasedStr::new("24FPS")        =>    Keyword::new(KeywordKind::VideoFrameRate),
    UncasedStr::new("29.97FPS")     =>    Keyword::new(KeywordKind::VideoFrameRate),
    UncasedStr::new("30FPS")        =>    Keyword::new(KeywordKind::VideoFrameRate),
    UncasedStr::new("60FPS")        =>    Keyword::new(KeywordKind::VideoFrameRate),
    UncasedStr::new("120FPS")       =>    Keyword::new(KeywordKind::VideoFrameRate),
    // Profile
    UncasedStr::new("Hi10")         =>    Keyword::new(KeywordKind::VideoProfile),
    UncasedStr::new("Hi10p")        =>    Keyword::new(KeywordKind::VideoProfile),
    UncasedStr::new("Hi444")        =>    Keyword::new(KeywordKind::VideoProfile),
    UncasedStr::new("Hi444P")       =>    Keyword::new(KeywordKind::VideoProfile),
    UncasedStr::new("Hi444PP")      =>    Keyword::new(KeywordKind::VideoProfile),
    // Quality
    UncasedStr::new("HD")           =>    Keyword::new(KeywordKind::VideoQuality),
    UncasedStr::new("SD")           =>    Keyword::new(KeywordKind::VideoQuality),
    UncasedStr::new("HQ")           =>    Keyword::new(KeywordKind::VideoQuality),
    UncasedStr::new("LQ")           =>    Keyword::new(KeywordKind::VideoQuality),
    // Resolution
    UncasedStr::new("480p")         =>    Keyword::unbounded(KeywordKind::VideoResolution),
    UncasedStr::new("720p")         =>    Keyword::unbounded(KeywordKind::VideoResolution),
    UncasedStr::new("1080p")        =>    Keyword::unbounded(KeywordKind::VideoResolution),
    UncasedStr::new("1440p")        =>    Keyword::unbounded(KeywordKind::VideoResolution),
    UncasedStr::new("2160p")        =>    Keyword::unbounded(KeywordKind::VideoResolution),
    UncasedStr::new("4K")           =>    Keyword::new(KeywordKind::VideoResolution),

    // Volume
    UncasedStr::new("Vol")          =>    Keyword::new(KeywordKind::Volume),
    UncasedStr::new("Volume")       =>    Keyword::new(KeywordKind::Volume),
};
