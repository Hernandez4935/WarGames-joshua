# WarGames/JOSHUA: Data Collection & Source Integration Strategy
## Comprehensive Data Acquisition Architecture
### Version 1.0.0 | October 2025

---

## Executive Summary

The WarGames/JOSHUA system requires continuous, reliable data collection from diverse sources to perform accurate nuclear risk assessments. This document specifies the complete data collection architecture, including source identification, collection methodologies, validation procedures, and integration strategies.

### Data Collection Principles

1. **Diversity**: Multiple redundant sources reduce single-point-of-failure risks
2. **Reliability**: Source quality scoring and validation ensure data integrity  
3. **Real-time**: Critical sources provide near-real-time updates for early warning
4. **Completeness**: Coverage across all risk factor categories
5. **Verifiability**: All data points traceable to original sources

---

## 1. Data Source Taxonomy

### 1.1 Primary Source Categories

```rust
pub enum DataSourceCategory {
    // Official government sources
    Government {
        country: String,
        agency: String,
        classification: SecurityLevel,
    },
    
    // Academic and research institutions
    Research {
        institution: String,
        specialty: ResearchArea,
        peer_reviewed: bool,
    },
    
    // International organizations
    International {
        organization: String,
        treaty_body: Option<String>,
    },
    
    // News and media
    Media {
        outlet: String,
        editorial_stance: EditorialPosition,
        fact_check_rating: f64,
    },
    
    // Social media intelligence
    Social {
        platform: Platform,
        account_type: AccountType,
        verified: bool,
    },
    
    // Specialized databases
    Database {
        name: String,
        update_frequency: Duration,
        coverage_areas: Vec<String>,
    },
}
```

### 1.2 Source Reliability Matrix

| Source Type | Reliability Score | Update Frequency | Coverage |
|-------------|------------------|------------------|----------|
| IAEA Official Reports | 0.95 | Monthly | Nuclear programs |
| SIPRI Database | 0.92 | Annual/Quarterly | Arsenal data |
| State Department | 0.90 | Daily | Diplomatic relations |
| Defense Intelligence | 0.88 | Weekly | Military activities |
| Reuters/AP News | 0.85 | Real-time | Global events |
| Academic Papers | 0.82 | Sporadic | Analysis/context |
| Think Tank Reports | 0.80 | Monthly | Expert analysis |
| Social Media (Verified) | 0.60 | Real-time | Sentiment/rhetoric |
| Social Media (Unverified) | 0.40 | Real-time | Early signals |

```rust
pub struct SourceReliability {
    pub base_score: f64,  // 0.0 to 1.0
    pub track_record: TrackRecord,
    pub verification_history: Vec<VerificationResult>,
    pub editorial_independence: f64,
    pub methodology_transparency: f64,
}

impl SourceReliability {
    /// Calculate dynamic reliability score
    pub fn current_score(&self) -> f64 {
        let track_record_weight = 0.4;
        let verification_weight = 0.3;
        let independence_weight = 0.2;
        let transparency_weight = 0.1;
        
        self.base_score *
        (track_record_weight * self.track_record.accuracy +
         verification_weight * self.verification_accuracy() +
         independence_weight * self.editorial_independence +
         transparency_weight * self.methodology_transparency)
    }
    
    fn verification_accuracy(&self) -> f64 {
        if self.verification_history.is_empty() {
            return 0.5;  // Unknown
        }
        
        let correct = self.verification_history.iter()
            .filter(|v| v.correct)
            .count();
        
        correct as f64 / self.verification_history.len() as f64
    }
}
```

---

## 2. News & Media Collection

### 2.1 RSS Feed Aggregation

```rust
pub struct RssFeedCollector {
    feeds: Vec<FeedSource>,
    cache: TimedCache<String, Vec<Article>>,
    deduplicator: ContentDeduplicator,
    parser: FeedParser,
}

impl RssFeedCollector {
    /// Collect from all RSS feeds
    pub async fn collect_all(&self) -> Result<Vec<Article>> {
        let mut all_articles = Vec::new();
        
        // Parallel feed fetching
        let futures: Vec<_> = self.feeds.iter()
            .map(|feed| self.collect_feed(feed))
            .collect();
        
        let results = futures::future::join_all(futures).await;
        
        for result in results {
            match result {
                Ok(articles) => all_articles.extend(articles),
                Err(e) => {
                    warn!("Feed collection failed: {}", e);
                    // Continue with other feeds
                }
            }
        }
        
        // Deduplicate articles
        let deduplicated = self.deduplicator.deduplicate(all_articles)?;
        
        Ok(deduplicated)
    }
    
    async fn collect_feed(&self, feed: &FeedSource) -> Result<Vec<Article>> {
        // Check cache first
        if let Some(cached) = self.cache.get(&feed.url) {
            return Ok(cached.clone());
        }
        
        // Fetch feed with timeout
        let response = reqwest::Client::new()
            .get(&feed.url)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;
        
        let content = response.text().await?;
        
        // Parse feed
        let articles = self.parser.parse_rss(&content, feed)?;
        
        // Cache results
        self.cache.insert(
            feed.url.clone(),
            articles.clone(),
            Duration::from_hours(1)
        );
        
        Ok(articles)
    }
}

pub struct FeedSource {
    pub url: String,
    pub outlet: String,
    pub reliability: f64,
    pub keywords: Vec<String>,
    pub language: String,
}

/// Nuclear-related RSS feeds
pub const PRIMARY_FEEDS: &[(&str, &str)] = &[
    ("https://www.reuters.com/arc/outboundfeeds/v3/collection/nuclear", "Reuters"),
    ("http://feeds.bbci.co.uk/news/world/rss.xml", "BBC World"),
    ("https://www.aljazeera.com/xml/rss/all.xml", "Al Jazeera"),
    ("https://english.alarabiya.net/rss.xml", "Al Arabiya"),
    ("http://tass.com/rss/v2.xml", "TASS"),
    ("http://www.xinhuanet.com/english/rss/worldrss.xml", "Xinhua"),
];
```

### 2.2 Content Deduplication

```rust
pub struct ContentDeduplicator {
    similarity_threshold: f64,
    hasher: SimHash,
}

impl ContentDeduplicator {
    pub fn deduplicate(&self, articles: Vec<Article>) -> Result<Vec<Article>> {
        let mut unique = Vec::new();
        let mut seen_hashes = HashSet::new();
        
        for article in articles {
            // Generate content hash
            let hash = self.hasher.hash_content(&article.content);
            
            // Check for near-duplicates
            if !self.is_duplicate(hash, &seen_hashes) {
                seen_hashes.insert(hash);
                unique.push(article);
            }
        }
        
        Ok(unique)
    }
    
    fn is_duplicate(&self, hash: u64, seen: &HashSet<u64>) -> bool {
        for &seen_hash in seen {
            let similarity = self.hasher.similarity(hash, seen_hash);
            if similarity > self.similarity_threshold {
                return true;
            }
        }
        false
    }
}

/// SimHash for fuzzy deduplication
pub struct SimHash;

impl SimHash {
    pub fn hash_content(&self, content: &str) -> u64 {
        // Extract features (words, bigrams, etc.)
        let features = self.extract_features(content);
        
        // Hash each feature and accumulate
        let mut v = [0i32; 64];
        
        for feature in features {
            let hash = Self::hash_feature(&feature);
            for i in 0..64 {
                if hash & (1 << i) != 0 {
                    v[i] += 1;
                } else {
                    v[i] -= 1;
                }
            }
        }
        
        // Generate final hash
        let mut result = 0u64;
        for i in 0..64 {
            if v[i] > 0 {
                result |= 1 << i;
            }
        }
        
        result
    }
    
    pub fn similarity(&self, hash1: u64, hash2: u64) -> f64 {
        let xor = hash1 ^ hash2;
        let hamming_distance = xor.count_ones();
        1.0 - (hamming_distance as f64 / 64.0)
    }
    
    fn extract_features(&self, content: &str) -> Vec<String> {
        // Tokenize and extract significant features
        content.split_whitespace()
            .filter(|w| w.len() > 3)
            .map(|w| w.to_lowercase())
            .collect()
    }
    
    fn hash_feature(feature: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        feature.hash(&mut hasher);
        hasher.finish()
    }
}
```

### 2.3 News API Integration

```rust
pub struct NewsAPIClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
    rate_limiter: RateLimiter,
}

impl NewsAPIClient {
    /// Search news with complex query
    pub async fn search_news(&self, query: NewsQuery) -> Result<Vec<Article>> {
        // Wait for rate limit
        self.rate_limiter.wait().await;
        
        let response = self.client
            .get(&format!("{}/v2/everything", self.base_url))
            .query(&[
                ("q", query.keywords.join(" OR ")),
                ("language", &query.language),
                ("sortBy", "relevancy"),
                ("pageSize", "100"),
            ])
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;
        
        let news_response: NewsAPIResponse = response.json().await?;
        
        Ok(news_response.articles)
    }
}

pub struct NewsQuery {
    pub keywords: Vec<String>,
    pub language: String,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
}

/// Nuclear-related search keywords
pub const NUCLEAR_KEYWORDS: &[&str] = &[
    "nuclear weapons", "nuclear war", "nuclear threat",
    "ICBM", "ballistic missile", "nuclear test",
    "uranium enrichment", "plutonium", "warhead",
    "doomsday clock", "nuclear doctrine", "deterrence",
    "arms control", "START treaty", "NPT",
    "nuclear submarine", "strategic forces",
    "tactical nuclear", "thermonuclear",
];

pub const GEOPOLITICAL_KEYWORDS: &[&str] = &[
    "Russia Ukraine", "Taiwan strait", "China military",
    "North Korea missile", "Iran nuclear",
    "India Pakistan", "Middle East conflict",
    "NATO", "airspace violation", "military exercise",
    "nuclear posture", "escalation", "crisis",
];
```

---

## 3. Research Institution Data Collection

### 3.1 SIPRI Integration

```rust
pub struct SipriCollector {
    client: reqwest::Client,
    base_url: String,
    cache: FileCache,
}

impl SipriCollector {
    /// Fetch nuclear forces data
    pub async fn fetch_nuclear_forces(&self) -> Result<NuclearForcesData> {
        // Check cache first (SIPRI updates annually)
        if let Some(cached) = self.cache.get("nuclear_forces")? {
            if cached.age() < Duration::from_days(30) {
                return Ok(cached.data);
            }
        }
        
        // Fetch from SIPRI website
        let html = self.client
            .get(&format!("{}/databases/nuclear-forces", self.base_url))
            .send()
            .await?
            .text()
            .await?;
        
        // Parse HTML tables
        let data = self.parse_nuclear_forces_tables(&html)?;
        
        // Cache results
        self.cache.set("nuclear_forces", &data, Duration::from_days(30))?;
        
        Ok(data)
    }
    
    fn parse_nuclear_forces_tables(&self, html: &str) -> Result<NuclearForcesData> {
        use scraper::{Html, Selector};
        
        let document = Html::parse_document(html);
        let table_selector = Selector::parse("table.forces-table").unwrap();
        let row_selector = Selector::parse("tr").unwrap();
        let cell_selector = Selector::parse("td").unwrap();
        
        let mut data = NuclearForcesData::default();
        
        for table in document.select(&table_selector) {
            for row in table.select(&row_selector).skip(1) {  // Skip header
                let cells: Vec<_> = row.select(&cell_selector)
                    .map(|c| c.text().collect::<String>().trim().to_string())
                    .collect();
                
                if cells.len() >= 5 {
                    let country = &cells[0];
                    let total: u32 = cells[1].parse().unwrap_or(0);
                    let deployed: u32 = cells[2].parse().unwrap_or(0);
                    let reserve: u32 = cells[3].parse().unwrap_or(0);
                    
                    data.arsenals.insert(country.clone(), Arsenal {
                        total_warheads: total,
                        deployed_strategic: deployed,
                        reserve_warheads: reserve,
                        last_updated: Utc::now(),
                    });
                }
            }
        }
        
        Ok(data)
    }
    
    /// Fetch latest yearbook excerpts
    pub async fn fetch_yearbook_summary(&self) -> Result<YearbookSummary> {
        let year = Utc::now().year();
        let url = format!(
            "{}/publications/sipri-yearbook-{}/executive-summary",
            self.base_url, year
        );
        
        let html = self.client.get(&url).send().await?.text().await?;
        
        self.parse_yearbook_summary(&html)
    }
}
```

### 3.2 Carnegie Endowment Integration

```rust
pub struct CarnegieCollector {
    client: reqwest::Client,
    api_endpoint: String,
}

impl CarnegieCollector {
    /// Fetch nuclear database entries
    pub async fn fetch_nuclear_database(&self) -> Result<Vec<DatabaseEntry>> {
        let response = self.client
            .get(&format!("{}/nuclear-db/api/entries", self.api_endpoint))
            .send()
            .await?;
        
        let entries: Vec<DatabaseEntry> = response.json().await?;
        
        Ok(entries)
    }
    
    /// Search for publications
    pub async fn search_publications(&self, query: &str) -> Result<Vec<Publication>> {
        let response = self.client
            .get(&format!("{}/publications/search", self.api_endpoint))
            .query(&[("q", query), ("topic", "nuclear-policy")])
            .send()
            .await?;
        
        let pubs: Vec<Publication> = response.json().await?;
        
        Ok(pubs)
    }
}
```

### 3.3 PDF Report Processing

```rust
pub struct PdfProcessor {
    extractor: PdfExtractor,
    table_parser: TableParser,
}

impl PdfProcessor {
    /// Extract text and tables from PDF
    pub async fn process_pdf(&self, pdf_url: &str) -> Result<PdfContent> {
        // Download PDF
        let pdf_bytes = self.download_pdf(pdf_url).await?;
        
        // Extract text
        let text = self.extractor.extract_text(&pdf_bytes)?;
        
        // Parse tables
        let tables = self.table_parser.extract_tables(&pdf_bytes)?;
        
        // Extract metadata
        let metadata = self.extractor.extract_metadata(&pdf_bytes)?;
        
        Ok(PdfContent {
            text,
            tables,
            metadata,
            source_url: pdf_url.to_string(),
        })
    }
    
    async fn download_pdf(&self, url: &str) -> Result<Vec<u8>> {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}

pub struct TableParser;

impl TableParser {
    /// Extract tables from PDF using tabula-style detection
    pub fn extract_tables(&self, pdf_bytes: &[u8]) -> Result<Vec<Table>> {
        // Use pdf-extract or similar library
        // Detect table boundaries
        // Parse cells and structure
        
        // Placeholder implementation
        Ok(Vec::new())
    }
}
```

---

## 4. Government Source Collection

### 4.1 State Department Monitoring

```rust
pub struct StateDepartmentCollector {
    client: reqwest::Client,
    base_url: String,
}

impl StateDepartmentCollector {
    /// Fetch latest press releases
    pub async fn fetch_press_releases(&self) -> Result<Vec<PressRelease>> {
        let html = self.client
            .get(&format!("{}/press-releases", self.base_url))
            .send()
            .await?
            .text()
            .await?;
        
        self.parse_press_releases(&html)
    }
    
    /// Fetch arms control compliance reports
    pub async fn fetch_compliance_reports(&self) -> Result<Vec<ComplianceReport>> {
        let reports_url = format!(
            "{}/bureaus-offices/bureaus/arms-control-verification-compliance/reports",
            self.base_url
        );
        
        let html = self.client.get(&reports_url).send().await?.text().await?;
        
        self.parse_compliance_reports(&html)
    }
}
```

### 4.2 IAEA Data Collection

```rust
pub struct IaeaCollector {
    client: reqwest::Client,
    base_url: String,
}

impl IaeaCollector {
    /// Fetch safeguards reports
    pub async fn fetch_safeguards_reports(&self) -> Result<Vec<SafeguardsReport>> {
        // IAEA publishes quarterly safeguards implementation reports
        let url = format!("{}/publications/documents/safeguards", self.base_url);
        
        let html = self.client.get(&url).send().await?.text().await?;
        
        self.parse_safeguards_reports(&html)
    }
    
    /// Fetch IAEA press releases and statements
    pub async fn fetch_statements(&self) -> Result<Vec<IaeaStatement>> {
        let url = format!("{}/newscenter/pressreleases", self.base_url);
        
        let rss = self.client.get(&format!("{}/rss", url))
            .send()
            .await?
            .text()
            .await?;
        
        self.parse_iaea_rss(&rss)
    }
}
```

---

## 5. Social Media Intelligence

### 5.1 Twitter/X Monitoring

```rust
pub struct TwitterCollector {
    client: TwitterAPIClient,
    monitored_accounts: Vec<String>,
    keywords: Vec<String>,
    sentiment_analyzer: SentimentAnalyzer,
}

impl TwitterCollector {
    /// Monitor official government accounts
    pub async fn monitor_official_accounts(&self) -> Result<Vec<Tweet>> {
        let mut all_tweets = Vec::new();
        
        for account in &self.monitored_accounts {
            let tweets = self.fetch_user_timeline(account, 100).await?;
            all_tweets.extend(tweets);
        }
        
        Ok(all_tweets)
    }
    
    /// Search for nuclear-related keywords
    pub async fn search_keywords(&self) -> Result<Vec<Tweet>> {
        let query = self.build_search_query();
        
        self.client.search_tweets(&query, 1000).await
    }
    
    fn build_search_query(&self) -> String {
        // Combine keywords with OR
        let keyword_query = self.keywords.join(" OR ");
        
        // Add filters for language and verified accounts
        format!(
            "({}) lang:en -is:retweet",
            keyword_query
        )
    }
    
    /// Analyze sentiment of tweets
    pub async fn analyze_sentiment(&self, tweets: &[Tweet]) -> SentimentReport {
        let mut positive = 0;
        let mut negative = 0;
        let mut neutral = 0;
        
        for tweet in tweets {
            let sentiment = self.sentiment_analyzer.analyze(&tweet.text);
            match sentiment {
                Sentiment::Positive => positive += 1,
                Sentiment::Negative => negative += 1,
                Sentiment::Neutral => neutral += 1,
            }
        }
        
        SentimentReport {
            positive_count: positive,
            negative_count: negative,
            neutral_count: neutral,
            overall_sentiment: self.calculate_overall_sentiment(
                positive, negative, neutral
            ),
        }
    }
}

/// Official accounts to monitor
pub const MONITORED_ACCOUNTS: &[&str] = &[
    // US Government
    "@StateDept", "@DeptofDefense", "@POTUS",
    
    // Russian Government  
    "@mfa_russia", "@mod_russia", "@KremlinRussia_E",
    
    // Chinese Government
    "@MFA_China", "@ChinaMission2UN",
    
    // International Organizations
    "@iaeaorg", "@UN", "@NATO",
    
    // Think Tanks and Experts
    "@ArmsControlNow", "@BulletinAtomic", "@RANDCorporation",
];
```

### 5.2 Reddit Monitoring

```rust
pub struct RedditCollector {
    client: RedditAPIClient,
    monitored_subreddits: Vec<String>,
}

impl RedditCollector {
    /// Monitor relevant subreddits
    pub async fn collect_discussions(&self) -> Result<Vec<RedditPost>> {
        let mut all_posts = Vec::new();
        
        for subreddit in &self.monitored_subreddits {
            let posts = self.fetch_hot_posts(subreddit, 100).await?;
            all_posts.extend(posts);
        }
        
        Ok(all_posts)
    }
    
    /// Search across Reddit for keywords
    pub async fn search_reddit(&self, query: &str) -> Result<Vec<RedditPost>> {
        self.client.search(query, 1000).await
    }
}

pub const MONITORED_SUBREDDITS: &[&str] = &[
    "worldnews",
    "geopolitics",
    "nuclearweapons",
    "credibledefense",
    "LessCredibleDefence",
    "foreignpolicy",
];
```

---

## 6. Data Validation & Quality Control

### 6.1 Cross-Source Verification

```rust
pub struct CrossSourceVerifier {
    min_sources: usize,
    agreement_threshold: f64,
}

impl CrossSourceVerifier {
    /// Verify claim across multiple sources
    pub async fn verify_claim(
        &self,
        claim: &DataClaim,
        sources: &[DataSource],
    ) -> VerificationResult {
        let mut supporting = 0;
        let mut contradicting = 0;
        let mut unclear = 0;
        
        for source in sources {
            match self.check_source_stance(claim, source).await {
                Stance::Supports => supporting += 1,
                Stance::Contradicts => contradicting += 1,
                Stance::Unclear => unclear += 1,
            }
        }
        
        let total = supporting + contradicting + unclear;
        let agreement_rate = supporting as f64 / total as f64;
        
        if total < self.min_sources {
            VerificationResult::Insufficient {
                sources_checked: total,
                required: self.min_sources,
            }
        } else if agreement_rate >= self.agreement_threshold {
            VerificationResult::Verified {
                confidence: agreement_rate,
                supporting_sources: supporting,
                total_sources: total,
            }
        } else {
            VerificationResult::Disputed {
                supporting: supporting,
                contradicting: contradicting,
                unclear: unclear,
            }
        }
    }
}
```

### 6.2 Data Quality Scoring

```rust
pub struct DataQualityScorer;

impl DataQualityScorer {
    /// Calculate quality score for data point
    pub fn score_data_point(&self, data: &DataPoint) -> DataQuality {
        let source_score = self.score_source(&data.source);
        let timeliness_score = self.score_timeliness(&data.timestamp);
        let verifiability_score = self.score_verifiability(data);
        let completeness_score = self.score_completeness(data);
        
        let overall_score = 
            0.35 * source_score +
            0.25 * timeliness_score +
            0.25 * verifiability_score +
            0.15 * completeness_score;
        
        DataQuality {
            overall_score,
            source_score,
            timeliness_score,
            verifiability_score,
            completeness_score,
            issues: self.identify_issues(data),
        }
    }
    
    fn score_source(&self, source: &DataSource) -> f64 {
        // Use source reliability matrix
        source.reliability.current_score()
    }
    
    fn score_timeliness(&self, timestamp: &DateTime<Utc>) -> f64 {
        let age = Utc::now() - *timestamp;
        
        // Decay function: 1.0 at 0 days, 0.5 at 30 days, 0.0 at 90+ days
        if age < Duration::zero() {
            0.0  // Future dates are suspicious
        } else if age < Duration::days(7) {
            1.0
        } else if age < Duration::days(30) {
            0.8
        } else if age < Duration::days(90) {
            0.5
        } else {
            0.2
        }
    }
    
    fn score_verifiability(&self, data: &DataPoint) -> f64 {
        let mut score = 0.0;
        
        // Has citations?
        if !data.citations.is_empty() {
            score += 0.3;
        }
        
        // Has corroborating sources?
        if data.corroborating_sources >= 2 {
            score += 0.4;
        }
        
        // Has primary source link?
        if data.primary_source_url.is_some() {
            score += 0.3;
        }
        
        score
    }
    
    fn score_completeness(&self, data: &DataPoint) -> f64 {
        let mut score = 1.0;
        
        // Deduct for missing fields
        if data.metadata.is_empty() {
            score -= 0.2;
        }
        if data.context.is_none() {
            score -= 0.3;
        }
        if data.uncertainty_estimate.is_none() {
            score -= 0.2;
        }
        
        score.max(0.0)
    }
}
```

---

## 7. Caching & Performance

### 7.1 Multi-Level Caching Strategy

```rust
pub struct CacheManager {
    memory_cache: LruCache<String, CachedData>,
    redis_cache: redis::Client,
    disk_cache: FileCache,
}

impl CacheManager {
    /// Get from cache with fallback strategy
    pub async fn get(&self, key: &str) -> Option<CachedData> {
        // 1. Try memory cache (fastest)
        if let Some(data) = self.memory_cache.get(key) {
            return Some(data.clone());
        }
        
        // 2. Try Redis cache
        if let Ok(Some(data)) = self.get_from_redis(key).await {
            // Populate memory cache
            self.memory_cache.put(key.to_string(), data.clone());
            return Some(data);
        }
        
        // 3. Try disk cache
        if let Ok(Some(data)) = self.get_from_disk(key) {
            // Populate higher-level caches
            self.memory_cache.put(key.to_string(), data.clone());
            let _ = self.set_in_redis(key, &data).await;
            return Some(data);
        }
        
        None
    }
    
    /// Store in all cache levels
    pub async fn set(&mut self, key: &str, data: CachedData, ttl: Duration) {
        // Store in all levels
        self.memory_cache.put(key.to_string(), data.clone());
        let _ = self.set_in_redis_with_ttl(key, &data, ttl).await;
        let _ = self.set_on_disk_with_ttl(key, &data, ttl);
    }
}
```

### 7.2 Rate Limiting

```rust
pub struct RateLimiter {
    limits: HashMap<String, RateLimit>,
    state: Arc<Mutex<HashMap<String, RateLimitState>>>,
}

pub struct RateLimit {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub requests_per_day: u32,
}

impl RateLimiter {
    /// Wait until request is allowed
    pub async fn wait(&self, key: &str) -> Result<()> {
        loop {
            let mut state = self.state.lock().await;
            let limit_state = state.entry(key.to_string())
                .or_insert_with(RateLimitState::new);
            
            if limit_state.can_proceed(self.get_limit(key)) {
                limit_state.record_request();
                break;
            }
            
            // Calculate wait time
            let wait_time = limit_state.time_until_next_allowed();
            drop(state);  // Release lock while waiting
            
            tokio::time::sleep(wait_time).await;
        }
        
        Ok(())
    }
}
```

---

## 8. Error Handling & Resilience

### 8.1 Retry Strategy

```rust
pub struct RetryStrategy {
    max_attempts: usize,
    base_delay: Duration,
    max_delay: Duration,
}

impl RetryStrategy {
    /// Execute with exponential backoff
    pub async fn execute<F, T, E>(&self, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> Pin<Box<dyn Future<Output = Result<T, E>>>>,
        E: std::fmt::Debug,
    {
        let mut attempt = 0;
        let mut delay = self.base_delay;
        
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempt += 1;
                    
                    if attempt >= self.max_attempts {
                        warn!("Max retries exceeded");
                        return Err(e);
                    }
                    
                    warn!("Attempt {} failed: {:?}, retrying in {:?}", 
                          attempt, e, delay);
                    
                    tokio::time::sleep(delay).await;
                    
                    // Exponential backoff with jitter
                    delay = (delay * 2)
                        .min(self.max_delay)
                        + Duration::from_millis(rand::random::<u64>() % 1000);
                }
            }
        }
    }
}
```

### 8.2 Graceful Degradation

```rust
pub struct DataCollectionOrchestrator {
    collectors: Vec<Box<dyn DataCollector>>,
    min_required_sources: usize,
}

impl DataCollectionOrchestrator {
    /// Collect with graceful degradation
    pub async fn collect_with_degradation(&self) -> Result<AggregatedData> {
        let mut results = Vec::new();
        let mut failures = Vec::new();
        
        // Try all collectors in parallel
        let futures: Vec<_> = self.collectors.iter()
            .map(|c| c.collect())
            .collect();
        
        let outcomes = futures::future::join_all(futures).await;
        
        for (i, outcome) in outcomes.into_iter().enumerate() {
            match outcome {
                Ok(data) => results.push(data),
                Err(e) => {
                    warn!("Collector {} failed: {}", i, e);
                    failures.push((i, e));
                }
            }
        }
        
        // Check if we have minimum required data
        if results.len() < self.min_required_sources {
            return Err(DataCollectionError::InsufficientSources {
                collected: results.len(),
                required: self.min_required_sources,
                failures,
            });
        }
        
        // Aggregate available data
        Ok(self.aggregate(results))
    }
}
```

---

## 9. Collection Schedule & Automation

### 9.1 Collection Frequency Matrix

| Source Type | Frequency | Priority | Timeout |
|-------------|-----------|----------|---------|
| Real-time Feeds (Twitter, News) | Continuous | High | 10s |
| News APIs | Hourly | High | 30s |
| Government Sites | Daily | Medium | 60s |
| Research Databases | Weekly | Medium | 120s |
| PDF Reports | On-demand | Low | 300s |

```rust
pub struct CollectionScheduler {
    schedules: HashMap<String, Schedule>,
    executor: tokio_cron_scheduler::JobScheduler,
}

impl CollectionScheduler {
    /// Initialize scheduled collection
    pub async fn init_schedules(&mut self) -> Result<()> {
        // Real-time sources (every 5 minutes)
        self.schedule_job(
            "*/5 * * * *",
            Box::new(|| self.collect_realtime_sources())
        ).await?;
        
        // News APIs (hourly)
        self.schedule_job(
            "0 * * * *",
            Box::new(|| self.collect_news_apis())
        ).await?;
        
        // Government sources (daily at 6 AM)
        self.schedule_job(
            "0 6 * * *",
            Box::new(|| self.collect_government_sources())
        ).await?;
        
        // Research databases (weekly on Monday)
        self.schedule_job(
            "0 8 * * 1",
            Box::new(|| self.collect_research_databases())
        ).await?;
        
        Ok(())
    }
}
```

---

## 10. Data Storage & Retrieval

### 10.1 Storage Schema

```rust
pub struct StorageManager {
    db: sqlx::PgPool,
}

impl StorageManager {
    /// Store collected data point
    pub async fn store_data_point(&self, data: &DataPoint) -> Result<Uuid> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            r#"
            INSERT INTO collected_data (
                id, source_id, category, content,
                timestamp, quality_score, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            id,
            data.source.id,
            data.category as _,
            data.content,
            data.timestamp,
            data.quality.overall_score,
            serde_json::to_value(&data.metadata)?
        )
        .execute(&self.db)
        .await?;
        
        Ok(id)
    }
    
    /// Query recent data by category
    pub async fn query_recent(
        &self,
        category: DataCategory,
        since: DateTime<Utc>,
        limit: i64
    ) -> Result<Vec<DataPoint>> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM collected_data
            WHERE category = $1 AND timestamp > $2
            ORDER BY timestamp DESC
            LIMIT $3
            "#,
            category as _,
            since,
            limit
        )
        .fetch_all(&self.db)
        .await?;
        
        // Convert rows to DataPoints
        self.rows_to_data_points(rows)
    }
}
```

---

## Conclusion

This comprehensive data collection strategy ensures the WarGames/JOSHUA system has access to reliable, timely, and diverse information sources. By combining automated collection, multi-source verification, quality scoring, and robust error handling, the system can maintain accurate situational awareness of global nuclear risks.

### Key Success Factors

1. **Source Diversity**: 50+ distinct sources across categories
2. **Real-time Capability**: Critical sources updated continuously
3. **Quality Control**: Multi-level verification and scoring
4. **Resilience**: Graceful degradation and retry strategies
5. **Performance**: Efficient caching and parallel collection

**Implementation Priority:**
1. Phase 1: News & Media (RSS + APIs)
2. Phase 2: Research Institutions (SIPRI, Carnegie)
3. Phase 3: Government Sources
4. Phase 4: Social Media Intelligence
5. Phase 5: Advanced Processing (PDFs, Images)

*"Knowledge is power. Accurate, timely knowledge is survival."*
