use std::env;
//global or general paramter for all codes
use lazy_static::lazy_static;
use std::sync::Mutex;
use anyhow::Context;
use surrealdb::iam::ResourceKind::Parameter as OtherParameter;

pub struct Databases {
    pub name: &'static str,
    pub username: &'static str,
    pub password: &'static str,//next in db
    pub ns: &'static str,
    pub db: &'static str,
    pub port: &'static str,
    pub url: &'static str, // Will be overridden by environment variables
    pub schema: &'static str,
}

impl Databases {
    /// Get production-ready database URL from environment or use default
    pub fn get_url(&self) -> String {
        env::var(format!("{}_URL", self.name))
            .unwrap_or_else(|_| format!("{}:{}", self.url, self.port))
    }
    
    /// Get production-ready database password from environment
    pub fn get_password(&self) -> String {
        env::var(format!("{}_PASSWORD", self.name))
            .unwrap_or_else(|_| self.password.to_string())
    }
    
    /// Get production-ready database username from environment
    pub fn get_username(&self) -> String {
        env::var(format!("{}_USERNAME", self.name))
            .unwrap_or_else(|_| self.username.to_string())
    }
    
    /// Get full connection string for PostgreSQL
    pub fn get_postgres_connection_string(&self) -> String {
        if self.name == "POSTGRES_DB" {
            format!(
                "host={} port={} user={} password={} dbname={} sslmode=disable",
                self.get_url(),
                self.port,
                self.get_username(),
                self.get_password(),
                self.db
            )
        } else {
            format!("{}:{}", self.get_url(), self.port)
        }
    }
}

/// Default database configurations (override in production via environment variables)
impl Databases {
    pub const SURREAL_DB: Databases = Databases {
        name: "SURREAL_DB",
        username: "root",
        password: "root", // Override with SURREAL_DB_PASSWORD
        ns: "pgd_ml_nmspace",
        db: "pgd_db",
        port: "8000",
        url: "127.0.0.1", // Override with SURREAL_DB_URL
        schema: "-",
    };
    
    pub const POSTGRES_DB: Databases = Databases {
        name: "POSTGRES_DB",
        username: "postgres", // Override with POSTGRES_DB_USERNAME
        password: "admin", // Override with POSTGRES_DB_PASSWORD
        ns: "-",
        db: "d_rag_lm", // Actual database name used in codebase
        port: "2345",
        url: "192.168.227.193", // Override with POSTGRES_DB_URL
        schema: "-",
    };
    
    pub const REDIS_DB: Databases = Databases {
        name: "REDIS_DB",
        username: "-",
        password: "-", // Override with REDIS_DB_PASSWORD
        ns: "-",
        db: "redis-oxide",
        port: "6379",
        url: "redis://127.0.0.1", // Override with REDIS_DB_URL
        schema: "-",
    };
}

pub struct UrlConnect {
    pub name: &'static str,
    pub port: &'static str,
    pub url: &'static str,
    pub path: &'static str,
}

impl UrlConnect {
    pub const LIB_SERVER: UrlConnect = UrlConnect {
        name: "LIB_SERVER",
        port: "3000",
        url: "http://0.0.0.0:3000",
        path: "-",
    };
}

pub struct LLM {
    pub code: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub size: &'static str,
    pub dim: &'static str,
}

impl LLM {
    /**
    * Ai Model
    */
    pub const AIS: LLM = LLM {
        code: "ais",
        version: "latest",
        description: "AIS",
        size: "G",
        dim: "2048"
    };

    pub const ASIST: LLM = LLM {
        code: "asist",
        version: "latest",
        description: "ASIST",
        size: "G",
        dim: "2048"
    };

    // pub const PLM: LLM = LLM {
    //     code: "PLM",
    //     version: "latest",
    //     description: "llm model PLM (Pegadaian Language Model)",
    //     size: "G",
    //     dim: "2048"
    // };

    // pub const PGLM: LLM = LLM {
    //     code: "PgLM",
    //     version: "latest",
    //     description: "llm model PgLM (Pegadaian Language Model)",
    //     size: "G",
    //     dim: "2048"
    // };

}
//note: buat loader / stream ambil dari parameter database

pub struct SizeDim {
    pub size: &'static str,
}

impl SizeDim {
    pub const SIZE_DIM_2048: SizeDim = SizeDim { size: "2048" };
    pub const SIZE_DIM_1024: SizeDim = SizeDim { size: "1024" };
    pub const SIZE_DIM_512: SizeDim = SizeDim { size: "512" };
}

pub struct MaxChar {
    pub max: &'static str,
}

impl MaxChar {
    //10.240 30.000
    pub const MAX_CHAR_IO_TXT_10240: MaxChar = MaxChar { max: "10240" };
    pub const MAX_CHAR_IO_TXT_30000: MaxChar = MaxChar { max: "30000" };
}


lazy_static! {
    pub static ref GLOBAL_ARRAY: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
}


pub struct LLMStatus {
    pub status: &'static str,
    pub description: &'static str,
}

impl LLMStatus {
    /**Flow Process Upload Ollama Server and Artifactory*/
    pub const LLM_READY: LLMStatus = LLMStatus { status: "1", description: "llm ready" };
    pub const INTERNAL_TEST_LLM_START: LLMStatus = LLMStatus { status: "2", description: "internal test llm" };
    pub const SUCCED_INTERNAL_TEST: LLMStatus = LLMStatus { status: "3", description: "succed internal test" };
    pub const SUCCED_BUILD_LIB: LLMStatus = LLMStatus { status: "4", description: "succed build lib (in rag engine)" };
    pub const ERR_INTERNAL_TEST: LLMStatus = LLMStatus { status: "0", description: "err internal test" };
    pub const ERR_BUILD_LIB: LLMStatus = LLMStatus { status: "00", description: "err build lib" };

    /**Flow Process create build LM*/
    pub const LLM_GEN_START: LLMStatus = LLMStatus { status: "LM.GEN.START", description: "Initialization Model Start" };
    pub const LLM_GEN_DATA_CHECK: LLMStatus = LLMStatus { status: "LM.GEN.DATA.CHECK", description: "Initialization Data" };
    pub const LLM_GEN_TOKENIZATION: LLMStatus = LLMStatus { status: "LM.GEN.TOKENIZATION", description: "Tokenization" };
    pub const LLM_GEN_EMBEDDING: LLMStatus = LLMStatus { status: "LM.GEN.EMBEDDING", description: "Embedding Layer" };
    pub const LLM_GEN_TRANSFORMER_BLOCK: LLMStatus = LLMStatus { status: "LM.GEN.TRANSFORMER.BLOCK", description: "Transformer Block" };
    pub const LLM_GEN_TRANSFORMER_POSITIONAL: LLMStatus = LLMStatus { status: "LM.GEN.TRANSFORMER.POSITIONAL", description: "Positional" };
    pub const LLM_GEN_TRANSFORMER_SERIALIZATION: LLMStatus = LLMStatus { status: "LM.GEN.TRANSFORMER.SERIALIZATION", description: "Serialization" };
    pub const LLM_GEN_MODEL: LLMStatus = LLMStatus { status: "LM.GEN.MODEL", description: "LLM Model" };
    pub const LLM_GEN_SAVE: LLMStatus = LLMStatus { status: "LM.GEN.SAVE", description: "LLM Model Save" };
    pub const LLM_GEN_QUANTIZE: LLMStatus = LLMStatus { status: "LM.GEN.QUANTIZE", description: "LLM Model Quantize" };
    pub const LLM_GEN_AFTER_QUANTIZE: LLMStatus = LLMStatus { status: "LM.GEN.AFTER.QUANTIZE", description: "LLM Model Quantize" };
    pub const LLM_GEN_AFTER_QUANTIZE_RESULT: LLMStatus = LLMStatus { status: "LM.GEN.AFTER.QUANTIZE.RESULT", description: "LLM Model Quantize RESULT" };

    /**Flow Process create build WASM LIB*/
    pub const WASM_LIB_START: LLMStatus = LLMStatus { status: "WASM.LIB.START", description: "Initialization Lib Start" };
    pub const WASM_LIB_BUILD: LLMStatus = LLMStatus { status: "WASM.LIB.BUILD", description: "Lib Build" };
    pub const WASM_LIB_FINISH: LLMStatus = LLMStatus { status: "WASM.LIB.FINISH", description: "Lib Created" };

}

pub struct FlowExecute {
    pub status: &'static str,
    pub description: &'static str,
}

impl FlowExecute {
    pub const FLOWSTART: FlowExecute = FlowExecute { status: "0", description: "start" };
    pub const FLOWDONE: FlowExecute = FlowExecute { status: "1", description: "done - finish" };
    pub const FLOW_IO_ERR: FlowExecute = FlowExecute { status: "3", description: "error" };
}

pub struct UserExecute {
    pub name: &'static str,
    pub description: &'static str,
}

impl UserExecute {
    pub const USER: UserExecute = UserExecute { name: "USER", description: "USER" };
    pub const SYSTEM: UserExecute = UserExecute { name: "SYSTEM", description: "SYSTEM" };
    pub const AGENT: UserExecute = UserExecute { name: "AGENT", description: "AGENT" };
}

pub struct Parameter {
    pub key: &'static str, pub value: &'static str, pub description: &'static str,
}

impl Parameter {
    /**Using DB Data**/
    pub const PARAMETER: Parameter = Parameter { key: "LM.BUILD.START", value: "0", description: "0: no process | 1: on process | 2: done process " };
    /**quantize process non DB Data**/
    pub const LLAMACPP_QUANTIZE: Parameter = Parameter { key: "LLAMACPP_QUANTIZE", value:"quantize", description: "quantize status" };
    /**quantize tool**/
    pub const LLAMACPP_RS_Q_OPTIM: Parameter = Parameter { key: "llamacpp_rs_q_optim", value:"quantize", description: "quantize status using llamacpp rs" };
    pub const OLLAMA_OPTIM: Parameter = Parameter { key: "ollama_optim", value:"quantize", description: "quantize status ollama" };
    pub const LLAMACPP_Q_OPTIM: Parameter = Parameter { key: "llamacpp_q_optim", value:"quantize", description: "quantize status using llamacpp" };

    ///Points,ParameterEnv:
    pub const parameter_dataset_financial: Parameter = Parameter { key: "parameter_dataset_financial", value:"parameter_dataset_financial", description: "qdrant vec parameter_dataset_financial" };
    pub const parameter_master_financial: Parameter = Parameter { key: "parameter_master_financial", value:"parameter_master_financial", description: "qdrant vec parameter_master_financial" };
    pub const parameter_detail_financial: Parameter = Parameter { key: "parameter_detail_financial", value:"parameter_financial", description: "qdrant vec parameter_detail_financial" };

}

pub fn ParameterEnv(param: &str) -> String {
    let mut result_parameter_env = String::new();

    if(Parameter::parameter_dataset_financial.key==param){
        let param = env::var("parameter_dataset_financial").context("parameter_dataset_financial not set");
        result_parameter_env = param.unwrap();
    } else if(Parameter::parameter_master_financial.key==param){
        let param = env::var("parameter_master_financial").context("parameter_master_financial not set");
        result_parameter_env = param.unwrap();
    } else if(Parameter::parameter_detail_financial.key==param){
        let param = env::var("parameter_detail_financial").context("parameter_detail_financial not set");
        result_parameter_env = param.unwrap();
    } else {
        let param = env::var("parameter_detail_financial").context("parameter_detail_financial not set");
        result_parameter_env = param.unwrap();
    }

    result_parameter_env

}



pub struct LMTYPE {
    pub name: &'static str,
    pub description: &'static str,
}

impl LMTYPE {
    /**Model Type**/
    pub const LLM: LMTYPE = LMTYPE { name: "LLM", description: "LLM" };
    pub const MOE: LMTYPE = LMTYPE { name: "MOE", description: "MOE" };
    pub const LCM: LMTYPE = LMTYPE { name: "LCM", description: "LCM" };
    pub const SLM: LMTYPE = LMTYPE { name: "SLM", description: "SLM" };
    pub const LAM: LMTYPE = LMTYPE { name: "LAM", description: "LAM" };
    pub const VLM: LMTYPE = LMTYPE { name: "VLM", description: "VLM" };
    pub const MLM: LMTYPE = LMTYPE { name: "MLM", description: "MLM" };
    pub const SAM: LMTYPE = LMTYPE { name: "SAM", description: "SAM" };

    /**Ekstensi**/
    pub const namefile_gguf: LMTYPE = LMTYPE { name: ".gguf", description: "gguf" };
    pub const namefile_bin: LMTYPE = LMTYPE { name: ".bin", description: "bin" };
    pub const namefile_sf: LMTYPE = LMTYPE { name: ".safetensor", description: "safetensor" };
    pub const namefile_onnx: LMTYPE = LMTYPE { name: ".onnx", description: "onnx" };

}

pub struct StatusProcess {
    pub status: &'static str,
    pub description: &'static str,
}

impl StatusProcess {
    pub const READY: StatusProcess = StatusProcess { status: "0", description: "ready" };
    pub const FIRST: StatusProcess = StatusProcess { status: "1", description: "first" };
    pub const APPROVE: StatusProcess = StatusProcess { status: "2", description: "approve" };
    pub const CANCEL: StatusProcess = StatusProcess { status: "3", description: "cancel" };
    pub const DONE: StatusProcess = StatusProcess { status: "4", description: "done" };

    pub const INITIALIZE: StatusProcess = StatusProcess { status: "INITIALIZE", description: "initialize" };
}

#[derive(Debug, Clone, Copy)]
pub enum LlamaQuantType {
    F16 = 1,
    Q4_0 = 2,
    Q4_1 = 3,
    Q5_0 = 6,
    Q5_1 = 7,
    Q8_0 = 8,
    Q2_K = 10,
    Q3_K_S = 11,
    Q3_K_M = 12,
    Q3_K_L = 13,
    Q4_K_S = 14,
    Q4_K_M = 15,
    Q5_K_S = 16,
    Q5_K_M = 17,
    Q6_K = 18,
    Q8_K = 19,
}
