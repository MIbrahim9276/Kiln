use clap::{Args, ValueEnum};


/* Shared Arguments */

#[derive(Args, Debug, Clone)]
pub struct TargetArg {
    #[arg(long, value_name = "NAME", help = "Select a project target")]
    pub target: Option<String>
}

#[derive(Args, Debug, Clone)]
pub struct TargetArgVec {
    #[arg(long, value_name = "NAME", help = "Select a project target")]
    pub target: Vec<String>
}


#[derive(Args, Debug, Clone)]
pub struct ProfileArg {
    #[arg(long, value_name = "NAME", default_value = "debug", help = "Use a profile")]
    pub profile: String
}


#[derive(Args, Debug, Clone)]
pub struct JobsArg {
    #[arg(long, value_name = "N", default_value_t = 0, help = "Number of concurrent jobs")]
    pub jobs: u32
}


#[derive(Args, Debug, Clone)]
pub struct FormatArg {
    #[arg(long, value_enum, default_value_t = Format::Text, help = "Presentation format")]
    pub format: Format
}

#[derive(Args, Debug, Clone)]
pub struct FormatWithDotArg {
    #[arg(long, value_enum, default_value_t = FormatWithDot::Text, help = "Presentation format")]
    pub format: FormatWithDot
}


#[derive(Args, Debug, Clone)]
pub struct NoBuildArg {
    #[arg(long, help = "If set, target is not built and is executed immediately")]
    pub no_build: bool
}


#[derive(Args, Debug, Clone)]
pub struct FreshArg {
    #[arg(long, help = "Discard build state for selected build context, configure, then rebuild")]
    pub fresh: bool
}



/* Enums */

#[derive(ValueEnum, Debug, Clone)]
pub enum Format {
    Text,
    Json
}

#[derive(ValueEnum, Debug, Clone)]
pub enum FormatWithDot {
    Text,
    Json,
    Dot
}