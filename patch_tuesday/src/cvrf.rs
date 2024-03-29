// curl -X GET --header 'Accept: application/json' 'https://api.msrc.microsoft.com/cvrf/v2.0/cvrf/2021-Jan' > response.json
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CVRFDocument {
    #[serde(rename = "DocumentTitle")]
    pub document_title: ValueField,

    #[serde(rename = "DocumentType")]
    pub document_type: ValueField,

    #[serde(rename = "DocumentPublisher")]
    pub document_publisher: DocumentPublisher,

    #[serde(rename = "DocumentTracking")]
    pub document_tracking: DocumentTracking,

    #[serde(rename = "DocumentNotes")]
    pub document_notes: Vec<DocumentNote>,

    // We skip deserializing this field since Microsoft fucks the structure every once in a while
    #[serde(rename = "ProductTree", skip)]
    pub product_tree: String,

    #[serde(rename = "Vulnerability")]
    pub vulnerability: Vec<Vulnerability>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ValueField {
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

#[derive(Deserialize)]
pub struct DocumentPublisher {
    #[serde(rename = "ContactDetails")]
    pub contact_details: ValueField,

    #[serde(rename = "IssuingAuthority")]
    pub issuing_authority: ValueField,

    #[serde(rename = "Type")]
    pub type_: i32,
}

#[derive(Deserialize)]
pub struct DocumentTracking {
    #[serde(rename = "Identification")]
    pub identification: Identification,

    #[serde(rename = "Status")]
    pub status: i32,

    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "RevisionHistory")]
    pub revision_history: Vec<Revision>,

    #[serde(rename = "InitialReleaseDate")]
    pub initial_release_date: String,

    #[serde(rename = "CurrentReleaseDate")]
    pub current_release_date: String,
}

#[derive(Deserialize)]
pub struct Identification {
    #[serde(rename = "ID")]
    pub id: ValueField,

    #[serde(rename = "Alias")]
    pub alias: ValueField,
}

#[derive(Deserialize)]
pub struct Revision {
    #[serde(rename = "Number")]
    pub number: String,

    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Description")]
    pub description: ValueField,
}

#[derive(Deserialize)]
pub struct DocumentNote {
    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "Audience")]
    pub audience: String,

    #[serde(rename = "Type")]
    pub type_: i32,

    #[serde(rename = "Ordinal")]
    pub ordinal: String,

    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Deserialize)]
pub struct Note {
    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "Type")]
    pub type_: i32,

    #[serde(rename = "Ordinal")]
    pub ordinal: String,

    #[serde(rename = "Value")]
    pub value: Option<String>,
}

#[derive(Deserialize)]
pub struct ProductTree {
    #[serde(rename = "Branch")]
    pub branch: Vec<Branch>,

    #[serde(rename = "FullProductName")]
    pub full_product_name: Vec<FullProductName>,
}

#[derive(Deserialize)]
pub struct Branch {
    #[serde(rename = "Items")]
    pub items: Vec<Item>,

    #[serde(rename = "Type")]
    pub type_: i32,

    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Deserialize)]
pub struct Item {
    #[serde(rename = "Items")]
    pub items: Vec<ProductIDValue>,

    #[serde(rename = "Type")]
    pub type_: i32,

    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Deserialize)]
pub struct ProductIDValue {
    #[serde(rename = "ProductID")]
    pub product_id: String,

    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Deserialize)]
pub struct FullProductName {
    #[serde(rename = "ProductID")]
    pub product_id: String,

    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Deserialize)]
pub struct Vulnerability {
    #[serde(rename = "Title")]
    pub title: ValueField,

    #[serde(rename = "Notes")]
    pub notes: Vec<Note>,

    #[serde(rename = "DiscoveryDateSpecified")]
    pub discovery_date_specified: bool,

    #[serde(rename = "ReleaseDateSpecified")]
    pub release_date_specified: bool,

    #[serde(rename = "CVE")]
    pub cve: String,

    #[serde(rename = "ProductStatuses")]
    pub product_statuses: Vec<ProductStatus>,

    #[serde(rename = "Threats")]
    pub threats: Vec<Threat>,

    #[serde(rename = "CVSSScoreSets")]
    pub cvss_score_sets: Vec<CVSSScoreSet>,

    #[serde(rename = "Remediations")]
    pub remediations: Vec<Remediation>,

    #[serde(rename = "Acknowledgments")]
    pub acknowledgments: Vec<Acknowledgment>,

    #[serde(rename = "Ordinal")]
    pub ordinal: String,

    #[serde(rename = "RevisionHistory")]
    pub revision_history: Vec<Revision>,
}

#[derive(Deserialize)]
pub struct ProductStatus {
    #[serde(rename = "ProductID")]
    pub product_id: Option<Vec<String>>,

    #[serde(rename = "Type")]
    pub type_: i32,
}

#[derive(Deserialize)]
pub struct Threat {
    #[serde(rename = "Description")]
    pub description: Option<ValueField>,

    #[serde(rename = "ProductID")]
    pub product_id: Option<Vec<String>>,

    #[serde(rename = "Type")]
    pub type_: i32,

    #[serde(rename = "DateSpecified")]
    pub date_specified: bool,
}

#[derive(Deserialize)]
pub struct CVSSScoreSet {
    #[serde(rename = "BaseScore")]
    pub base_score: f64,

    #[serde(rename = "TemporalScore")]
    pub temporal_score: f64,

    #[serde(rename = "Vector")]
    pub vector: String,

    #[serde(rename = "ProductID")]
    pub product_id: Vec<String>,
}

#[derive(Deserialize)]
pub struct Remediation {
    #[serde(rename = "Description")]
    pub description: ValueField,

    #[serde(rename = "URL")]
    pub url: Option<String>,

    #[serde(rename = "ProductID")]
    pub product_id: Option<Vec<String>>,

    #[serde(rename = "Type")]
    pub type_: i32,

    #[serde(rename = "DateSpecified")]
    pub date_specified: bool,

    #[serde(rename = "AffectedFiles")]
    pub affected_files: Vec<AffectedFile>,

    #[serde(rename = "RestartRequired")]
    pub restart_required: Option<ValueField>,

    #[serde(rename = "SubType")]
    pub sub_type: Option<String>,

    #[serde(rename = "FixedBuild")]
    pub fixed_build: Option<String>,
}

#[derive(Deserialize)]
pub struct AffectedFile {
    #[serde(rename = "FileName")]
    pub file_name: String,

    #[serde(rename = "FileLastModified")]
    pub file_last_modified: String,
}

#[derive(Deserialize)]
pub struct Acknowledgment {
    #[serde(rename = "Name")]
    pub name: Vec<ValueField>,

    #[serde(rename = "URL")]
    pub url: Vec<String>,
}
