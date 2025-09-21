use super::platform::{Architecture, OS};

pub struct Release {
    pub id: u32,
    pub version: String,
    pub size: u64,
    pub sha256: String,
    pub architecture: Architecture,
    pub os: OS,
    pub servers: Vec<String>,
}

pub fn get_releases() -> Vec<Release> {
    vec![
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 271306890,
            sha256: "dd5331d4aafbd9c736035972c3e99fc82116f28cbf6f76d701edf3794c532167".to_string(),
            architecture: Architecture::Arm64,
            os: OS::Android,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 271124256,
            sha256: "c7b82901192a7143949d519b06e7898faf34a32e166e6cda83ac0565f1ed4bba".to_string(),
            architecture: Architecture::Amd64,
            os: OS::Darwin,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 272503250,
            sha256: "a040bb7a4f397ddff9139f2c7354b471a0ffecb66662020d62423622048e7384".to_string(),
            architecture: Architecture::Arm64,
            os: OS::Darwin,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 270393551,
            sha256: "00cf299f56c54c19bfe5117ec3c56a8a26ffea195beffde0dac9c301ac185ba6".to_string(),
            architecture: Architecture::I386,
            os: OS::Freebsd,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 271022984,
            sha256: "81aedd65e89418d083ad4eea8259d1014f78e740441f54fb5e2d966f3701eae5".to_string(),
            architecture: Architecture::Amd64,
            os: OS::Freebsd,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 270320014,
            sha256: "79c8f751807919cc1cc7ee10b40100cf888ed81f6cd7497e0af3098e8bdbbad3".to_string(),
            architecture: Architecture::Arm,
            os: OS::Freebsd,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 270328566,
            sha256: "591e2156cff692d6e956195b0b7a135448c530207cc812292f495afa781f8290".to_string(),
            architecture: Architecture::Arm64,
            os: OS::Freebsd,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 270522389,
            sha256: "56a74c518205698ef85a5e2fc0197efd1832ac1a1a4b4cc7c945dbca8b4f8866".to_string(),
            architecture: Architecture::I386,
            os: OS::Linux,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 271165402,
            sha256: "50cf713d3e8eb7e78f44774a6391965a6611ed39428fc671a4bb980dd278cbd8".to_string(),
            architecture: Architecture::Amd64,
            os: OS::Linux,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 270384173,
            sha256: "e78a22b9754733c8d8476031f05df9d3ce5a7c442e68c2da90512c587b816cbe".to_string(),
            architecture: Architecture::Arm,
            os: OS::Linux,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 270396506,
            sha256: "29ab279ff9b45f57ee7263a1ea3a0b1b39707d8e2e64d17fabb33f92797656ca".to_string(),
            architecture: Architecture::Arm64,
            os: OS::Linux,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 270568439,
            sha256: "f780f61d77810cf169eccd544541f6bf11155ac2a2bab1907ba4f15f4235d95b".to_string(),
            architecture: Architecture::Riscv64,
            os: OS::Linux,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 270887936,
            sha256: "4fc66457008cd8442f104085203651f465fd215467cb6e98d01cb1fefe4f7865".to_string(),
            architecture: Architecture::I386,
            os: OS::Windows,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 271423488,
            sha256: "03177f3c083362a0abca177cdfd7aa806199b59c5be1919e625e9ea7dda0d964".to_string(),
            architecture: Architecture::Amd64,
            os: OS::Windows,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 270647296,
            sha256: "b2a9985b7396245077b0e0abb44eb9e7596526217ed4947d9b8b5e1132e55d9d".to_string(),
            architecture: Architecture::Arm,
            os: OS::Windows,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
        Release {
            id: 603,
            version: "v0.6.3".to_string(),
            size: 270575616,
            sha256: "c92bbb51635aedd990724f1979444618a89280699a4aea80d251e9035ffc94a7".to_string(),
            architecture: Architecture::Arm64,
            os: OS::Windows,
            servers: vec![
                "https://github.com/neurlang/goruut/releases/download/v0.6.3/".to_string(),
                "https://www.hashtron.cloud/dl/v0.6.3/".to_string(),
            ],
        },
    ]
}
