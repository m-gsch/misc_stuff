pub mod cvrf;
use clap::ValueEnum;
use std::{fmt, str};

#[derive(clap::ValueEnum, Clone, PartialEq)]
pub enum Severity {
    Critical,
    Important,
    Moderate,
    High,
    Medium,
    Low,
    None,
}

impl str::FromStr for Severity {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ValueEnum::from_str(s, true)
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Critical => "Critical",
                Self::Important => "Important",
                Self::Moderate => "Moderate",
                Self::High => "High",
                Self::Medium => "Medium",
                Self::Low => "Low",
                Self::None => "",
            }
        )
    }
}

pub enum Impact {
    RemoteCodeExecution,
    EscalationOfPrivilege,
    DenialOfService,
    SecurityFeatureBypass,
    InformationDisclosure,
    Spoofing,
}

#[allow(non_camel_case_types)]
#[derive(ValueEnum, Clone, Copy, PartialEq, Debug)]
pub enum Product {
    All,
    Win10_1809_x64 = 11569,
    Win11_22H2_x64 = 12086,
} // Add more products as necessary :)

impl str::FromStr for Product {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "11569" => Ok(Product::Win10_1809_x64),
            "12086" => Ok(Product::Win11_22H2_x64),
            _ => Err("Invalid product"),
        }
    }
}

#[derive(Clone)]
pub struct Vulnerability {
    pub title: String,
    pub cve: String,
    pub severity: Severity,
    pub cvss: Option<f64>,
    pub impact: String,
    pub description: Option<String>,
    pub acknowledgements: Option<String>,
    pub public: bool,
    pub exploited: bool,
    pub affected_products: Vec<String>,
}

impl From<&cvrf::Vulnerability> for Vulnerability {
    fn from(item: &cvrf::Vulnerability) -> Self {
        let title = item.title.value.clone().unwrap_or_default();
        let cve = item.cve.clone();
        let severity = item
            .threats
            .iter()
            .find(|threat| threat.type_ == 3)
            .and_then(|note| note.description.clone().unwrap().value)
            .unwrap_or("None".to_owned())
            .parse()
            .unwrap();
        let cvss = item
            .cvss_score_sets
            .get(0)
            .and_then(|cvss_set| Some(cvss_set.base_score));
        let impact = item
            .threats
            .iter()
            .find(|threat| threat.type_ == 0)
            .and_then(|note| note.description.clone().unwrap().value)
            .unwrap_or_default();
        let description = item
            .notes
            .iter()
            .find(|note| note.title == "Description")
            .and_then(|note| note.value.clone());
        let acknowledgements: Option<String> = item
            .acknowledgments
            .iter()
            .flat_map(|ack| ack.name.iter())
            .map(|field| field.value.clone())
            .collect();
        let vuln_exploitability = item
            .threats
            .iter()
            .find(|threat| threat.type_ == 1)
            .and_then(|note| note.description.clone().unwrap().value)
            .unwrap_or_default();
        let exploitability_fields: Vec<&str> = vuln_exploitability.split(';').collect();
        // println!("{:#?}", exploitability_fields); todo! some only have "DOS:N/A"
        let public = exploitability_fields.get(0).unwrap_or(&"").contains("Yes");
        let exploited = exploitability_fields.get(1).unwrap_or(&"").contains("Yes");
        let affected_products = item
            .product_statuses
            .iter()
            .find(|product_status| product_status.type_ == 3)
            .and_then(|product_status| product_status.product_id.clone())
            .unwrap_or_default();
        Vulnerability {
            title,
            cve,
            severity,
            cvss,
            impact,
            description,
            acknowledgements,
            public,
            exploited,
            affected_products,
        }
    }
}

impl fmt::Display for Vulnerability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.title)?;
        write!(f, "{}\n", self.cve)?;
        write!(f, "Severity: {}\n", self.severity)?;
        if let Some(cvss) = &self.cvss {
            write!(f, "CVSS: {cvss}\n")?;
        }
        write!(f, "Impact: {}\n", self.impact)?;
        if let Some(description) = &self.description {
            write!(f, "Description: {description}\n")?;
        }
        write!(f, "Publicly Disclosed: {}\n", self.public)?;
        write!(f, "Exploited: {}\n", self.exploited)?;
        if let Some(acknowledgements) = &self.acknowledgements {
            write!(f, "Acknowledgments: {acknowledgements}\n")?;
        }

        write!(f, "{}", "-".repeat(8))
    }
}
