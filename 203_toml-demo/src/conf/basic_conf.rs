use super::*;

#[derive(Debug)]
pub struct Database {
    pub(crate) adapter: String, // 适配器
    pub(crate) db_name: String, // 数据库名称
    pub(crate) pool: u32, // 数据库池
}

#[derive(Debug)]
pub struct BasicConfig {
    pub environment: Environment,
    pub address: String,
    pub port: u16,
    pub database: Option<Database>,
    pub workers: Option<u16>, // 可选的配置，将来启动的任务数
    pub(crate) config_file_path: Option<PathBuf>, // 路径也是可选
    pub(crate) root_path: Option<PathBuf>, // 根路径也是可选的，用Option来表示
}

impl BasicConfig {

    pub fn new(env: Environment) -> Self {
        Self::default(env)
    }

    pub(crate) fn default(env: Environment) -> Self {
        let default_workers = (num_cpus::get() * 2) as u16;
        let default_config = BasicConfig {
                    environment: Development,
                    address: "localhost".to_string(),
                    port: 8000,
                    database: None,
                    workers: Some(default_workers),
                    config_file_path: None,
                    root_path: None,
        };
        
        match env {
            Development => {
                BasicConfig {
                    environment: Development,
                    ..default_config
                }
            }
            Staging => {
                BasicConfig {
                    environment: Staging,
                    ..default_config
                }
            }
            Production => {
                BasicConfig {
                    environment: Production,
                    ..default_config
                }
            }
        }
    }

    // 设置root路径
    pub fn set_root<P: AsRef<Path>>(&mut self, path: P) {
        self.root_path = Some(path.as_ref().into());
    }

    // 从默认配置文件中读取值
    pub(crate) fn default_from<P>(env: Environment, path: P) -> Result<Self>
        where P: AsRef<Path>
    {
        let mut config = BasicConfig::default(env);

        let config_file_path = path.as_ref().to_path_buf();
        if let Some(parent) = config_file_path.parent() {
            config.set_root(parent);
        } else {
            let msg = "Configuration files must be rooted in a directory.";
            return Err(ConfigError::BadFilePath(config_file_path.clone(), msg)); // 定义了一些读取的错误
        }

        config.config_file_path = Some(config_file_path);
        Ok(config)
    }
}

// 为了hashmap在insert时需要实现PartialEq
impl PartialEq for BasicConfig {
    fn eq(&self, other: &BasicConfig) -> bool {
        self.address == other.address
            && self.port == other.port
            && self.workers == other.workers
    }
}