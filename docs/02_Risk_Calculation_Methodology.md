# WarGames/JOSHUA: Risk Calculation Methodology & Algorithms
## Statistical Foundations for Nuclear War Risk Assessment
### Version 1.0.0 | October 2025

---

## Executive Summary

This document provides the complete mathematical and statistical methodology for calculating nuclear war risk in the WarGames/JOSHUA system. The approach combines expert-informed factor weighting, Bayesian probabilistic modeling, time-series analysis, and Monte Carlo simulation to produce robust, scientifically defensible risk assessments.

### Core Principles

1. **Multi-Factor Integration**: Risk emerges from complex interactions of 50+ distinct factors
2. **Uncertainty Quantification**: All estimates include confidence intervals and uncertainty bounds
3. **Historical Validation**: Methodologies validated against known historical events
4. **Statistical Rigor**: Proper hypothesis testing, significance levels, and error propagation
5. **Transparency**: All calculations are auditable and explainable

---

## Table of Contents

1. [Risk Factor Framework](#risk-factor-framework)
2. [Weighted Scoring System](#weighted-scoring-system)
3. [Bayesian Risk Adjustment](#bayesian-risk-adjustment)
4. [Time-Series Analysis](#time-series-analysis)
5. [Monte Carlo Simulation](#monte-carlo-simulation)
6. [Uncertainty Quantification](#uncertainty-quantification)
7. [Historical Calibration](#historical-calibration)
8. [Implementation Specifications](#implementation-specifications)

---

## 1. Risk Factor Framework

### 1.1 Factor Taxonomy

Nuclear war risk factors are organized into seven primary categories, each containing multiple sub-factors:

```rust
pub enum RiskCategory {
    // 15% of total weight
    NuclearArsenalChanges {
        modernization_programs: Vec<ModernizationActivity>,
        warhead_count_changes: HashMap<Country, i32>,
        delivery_system_advances: Vec<TechnologyAdvance>,
        tactical_deployments: Vec<Deployment>,
    },
    
    // 15% of total weight
    DoctrineAndPosture {
        threshold_changes: Vec<DoctrineChange>,
        alert_status: HashMap<Country, AlertLevel>,
        no_first_use_changes: Vec<PolicyChange>,
        declaratory_policy_shifts: Vec<Statement>,
    },
    
    // 20% of total weight
    RegionalConflicts {
        active_conflicts: Vec<Conflict>,
        territorial_disputes: Vec<Dispute>,
        proxy_warfare: Vec<ProxyConflict>,
        military_incidents: Vec<Incident>,
    },
    
    // 10% of total weight
    LeadershipAndRhetoric {
        nuclear_threats: Vec<Threat>,
        leadership_stability: HashMap<Country, Stability>,
        domestic_pressure: HashMap<Country, Pressure>,
        cognitive_factors: Vec<PsychologicalFactor>,
    },
    
    // 15% of total weight
    TechnicalIncidents {
        airspace_violations: Vec<Violation>,
        false_alarms: Vec<FalseAlarm>,
        cyber_attacks: Vec<CyberIncident>,
        accidents: Vec<Accident>,
    },
    
    // 10% of total weight
    CommunicationBreakdown {
        hotline_failures: Vec<HotlineEvent>,
        data_exchange_suspension: Vec<Suspension>,
        diplomatic_expulsions: Vec<Expulsion>,
        verification_breakdown: Vec<VerificationEvent>,
    },
    
    // 10% of total weight
    EmergingTechnology {
        ai_integration: Vec<AIDeployment>,
        hypersonic_deployment: Vec<HypersonicSystem>,
        space_weaponization: Vec<SpaceWeapon>,
        cyber_capabilities: Vec<CyberCapability>,
    },
    
    // 5% of total weight
    EconomicFactors {
        sanctions_pressure: Vec<SanctionRegime>,
        resource_competition: Vec<ResourceDispute>,
        supply_chain_warfare: Vec<SupplyChainEvent>,
        economic_instability: Vec<EconomicCrisis>,
    },
}
```

### 1.2 Factor Quantification

Each risk factor is quantified on a normalized scale [0, 1] where:
- **0.0**: Factor represents no risk (ideal baseline)
- **0.25**: Low risk / normal peacetime conditions
- **0.50**: Moderate risk / elevated tensions
- **0.75**: High risk / crisis conditions
- **1.0**: Critical risk / imminent threat

```rust
pub struct RiskFactor {
    pub category: RiskCategory,
    pub name: String,
    pub value: f64,  // [0.0, 1.0]
    pub confidence: ConfidenceLevel,  // How certain are we?
    pub data_sources: Vec<DataSource>,
    pub timestamp: DateTime<Utc>,
    pub trend: TrendIndicator,  // Improving, Stable, Deteriorating
}

#[derive(Debug, Clone, Copy)]
pub enum ConfidenceLevel {
    VeryLow,   // <50% confidence
    Low,       // 50-70%
    Moderate,  // 70-85%
    High,      // 85-95%
    VeryHigh,  // >95%
}

impl ConfidenceLevel {
    pub fn to_numeric(&self) -> f64 {
        match self {
            Self::VeryLow => 0.40,
            Self::Low => 0.60,
            Self::Moderate => 0.775,
            Self::High => 0.90,
            Self::VeryHigh => 0.975,
        }
    }
}
```

### 1.3 Factor Evaluation Functions

Each factor type has a specific evaluation function:

```rust
pub trait FactorEvaluator {
    /// Evaluate the risk contribution of this factor
    fn evaluate(&self, context: &GlobalContext) -> FactorEvaluation;
    
    /// Determine confidence in this evaluation
    fn assess_confidence(&self, data_quality: &DataQuality) -> ConfidenceLevel;
    
    /// Identify data gaps or uncertainties
    fn identify_uncertainties(&self) -> Vec<Uncertainty>;
}

pub struct FactorEvaluation {
    pub value: f64,
    pub confidence: ConfidenceLevel,
    pub reasoning: String,
    pub supporting_evidence: Vec<Evidence>,
    pub contrary_evidence: Vec<Evidence>,
    pub uncertainty_bounds: (f64, f64),
}
```

#### Example: Arsenal Modernization Evaluator

```rust
impl FactorEvaluator for ArsenalModernizationFactor {
    fn evaluate(&self, context: &GlobalContext) -> FactorEvaluation {
        // Assess multiple dimensions of arsenal changes
        let warhead_change_score = self.evaluate_warhead_changes(context);
        let delivery_score = self.evaluate_delivery_systems(context);
        let readiness_score = self.evaluate_readiness_posture(context);
        let technology_score = self.evaluate_technology_advances(context);
        
        // Weighted combination
        let value = 
            0.25 * warhead_change_score +
            0.30 * delivery_score +
            0.25 * readiness_score +
            0.20 * technology_score;
        
        // Assess confidence based on data quality
        let confidence = self.assess_confidence(&context.data_quality);
        
        // Calculate uncertainty bounds (95% CI)
        let stderr = self.estimate_standard_error(context);
        let uncertainty_bounds = (
            (value - 1.96 * stderr).max(0.0),
            (value + 1.96 * stderr).min(1.0),
        );
        
        FactorEvaluation {
            value,
            confidence,
            reasoning: self.generate_reasoning(),
            supporting_evidence: self.collect_supporting_evidence(),
            contrary_evidence: self.collect_contrary_evidence(),
            uncertainty_bounds,
        }
    }
}
```

---

## 2. Weighted Scoring System

### 2.1 Base Weighted Score Calculation

The base risk score aggregates all factors using expert-informed weights:

```rust
pub struct WeightedScorer {
    category_weights: HashMap<RiskCategory, f64>,
    factor_weights: HashMap<String, f64>,
    interaction_matrix: InteractionMatrix,
}

impl WeightedScorer {
    /// Calculate weighted risk score
    pub fn calculate_base_score(&self, factors: &[RiskFactor]) -> f64 {
        let mut score = 0.0;
        let mut total_weight = 0.0;
        
        // Group factors by category
        let by_category = self.group_by_category(factors);
        
        for (category, category_factors) in by_category {
            // Get category weight
            let cat_weight = self.category_weights
                .get(&category)
                .copied()
                .unwrap_or(0.0);
            
            // Calculate category score
            let cat_score = self.calculate_category_score(&category_factors);
            
            // Weight by confidence
            let confidence_weight = self.calculate_confidence_weight(&category_factors);
            
            score += cat_weight * cat_score * confidence_weight;
            total_weight += cat_weight * confidence_weight;
        }
        
        // Normalize by total weight
        if total_weight > 0.0 {
            score / total_weight
        } else {
            0.0
        }
    }
    
    fn calculate_category_score(&self, factors: &[RiskFactor]) -> f64 {
        if factors.is_empty() {
            return 0.0;
        }
        
        let mut score = 0.0;
        let mut weight_sum = 0.0;
        
        for factor in factors {
            let factor_weight = self.factor_weights
                .get(&factor.name)
                .copied()
                .unwrap_or(1.0 / factors.len() as f64);
            
            score += factor_weight * factor.value;
            weight_sum += factor_weight;
        }
        
        score / weight_sum
    }
    
    fn calculate_confidence_weight(&self, factors: &[RiskFactor]) -> f64 {
        // Average confidence across all factors in category
        let total_confidence: f64 = factors
            .iter()
            .map(|f| f.confidence.to_numeric())
            .sum();
        
        total_confidence / factors.len() as f64
    }
}
```

### 2.2 Category Weights

Default category weights based on expert assessment and historical analysis:

| Category | Weight | Justification |
|----------|--------|---------------|
| Nuclear Arsenal Changes | 15% | Direct capability changes are slow but significant |
| Doctrine and Posture | 15% | Policy changes directly affect use likelihood |
| Regional Conflicts | 20% | Most immediate escalation pathway |
| Leadership and Rhetoric | 10% | Influences decision-making but less predictive |
| Technical Incidents | 15% | Accidents can trigger rapid escalation |
| Communication Breakdown | 10% | Critical for managing crises |
| Emerging Technology | 10% | Growing importance but still developing |
| Economic Factors | 5% | Indirect but can create desperation |

```rust
pub const DEFAULT_CATEGORY_WEIGHTS: &[(RiskCategory, f64)] = &[
    (RiskCategory::NuclearArsenalChanges, 0.15),
    (RiskCategory::DoctrineAndPosture, 0.15),
    (RiskCategory::RegionalConflicts, 0.20),
    (RiskCategory::LeadershipAndRhetoric, 0.10),
    (RiskCategory::TechnicalIncidents, 0.15),
    (RiskCategory::CommunicationBreakdown, 0.10),
    (RiskCategory::EmergingTechnology, 0.10),
    (RiskCategory::EconomicFactors, 0.05),
];
```

### 2.3 Weight Sensitivity Analysis

Weights are validated through sensitivity analysis:

```rust
pub struct SensitivityAnalyzer {
    base_weights: HashMap<RiskCategory, f64>,
    perturbation_range: f64,  // e.g., Â±20%
}

impl SensitivityAnalyzer {
    /// Perform sensitivity analysis on category weights
    pub fn analyze_sensitivity(
        &self,
        factors: &[RiskFactor],
    ) -> SensitivityResults {
        let base_score = self.calculate_with_weights(&self.base_weights, factors);
        let mut results = Vec::new();
        
        for (category, base_weight) in &self.base_weights {
            // Test weight perturbations
            for delta in [-0.2, -0.1, 0.1, 0.2].iter() {
                let mut perturbed_weights = self.base_weights.clone();
                let new_weight = (base_weight * (1.0 + delta))
                    .max(0.0)
                    .min(1.0);
                perturbed_weights.insert(*category, new_weight);
                
                // Renormalize
                self.normalize_weights(&mut perturbed_weights);
                
                let perturbed_score = self.calculate_with_weights(
                    &perturbed_weights,
                    factors
                );
                
                results.push(SensitivityResult {
                    category: *category,
                    weight_delta: *delta,
                    score_delta: perturbed_score - base_score,
                    elasticity: (perturbed_score - base_score) / 
                               (base_score * delta),
                });
            }
        }
        
        SensitivityResults {
            base_score,
            results,
            most_sensitive: self.identify_most_sensitive(&results),
        }
    }
}
```

---

## 3. Bayesian Risk Adjustment

### 3.1 Bayesian Network Structure

A Bayesian network models conditional dependencies between risk factors:

```rust
pub struct BayesianNetwork {
    nodes: HashMap<String, BayesianNode>,
    edges: Vec<BayesianEdge>,
    conditional_prob_tables: HashMap<String, CPT>,
}

pub struct BayesianNode {
    pub name: String,
    pub factor: RiskFactor,
    pub parents: Vec<String>,
    pub children: Vec<String>,
    pub state_space: Vec<NodeState>,
}

pub struct BayesianEdge {
    pub from: String,
    pub to: String,
    pub strength: f64,  // Edge weight / correlation
}

pub struct CPT {
    // Conditional Probability Table
    // P(Node | Parents)
    pub node_name: String,
    pub table: HashMap<ParentStates, Vec<f64>>,
}
```

### 3.2 Network Construction

The network is constructed from historical correlation analysis:

```rust
impl BayesianNetwork {
    /// Learn network structure from historical data
    pub fn learn_structure(
        historical_assessments: &[Assessment],
    ) -> Result<Self> {
        // 1. Calculate pairwise correlations
        let correlations = Self::calculate_correlations(historical_assessments)?;
        
        // 2. Identify strong dependencies (|r| > 0.3)
        let edges = correlations
            .iter()
            .filter(|(_, _, corr)| corr.abs() > 0.3)
            .map(|(from, to, strength)| BayesianEdge {
                from: from.clone(),
                to: to.clone(),
                strength: *strength,
            })
            .collect();
        
        // 3. Construct DAG (prevent cycles)
        let dag = Self::construct_dag(edges)?;
        
        // 4. Learn conditional probability tables
        let cpts = Self::learn_cpts(&dag, historical_assessments)?;
        
        Ok(BayesianNetwork {
            nodes: dag.nodes,
            edges: dag.edges,
            conditional_prob_tables: cpts,
        })
    }
    
    /// Calculate correlation matrix from historical data
    fn calculate_correlations(
        assessments: &[Assessment],
    ) -> Result<Vec<(String, String, f64)>> {
        let mut correlations = Vec::new();
        
        // Extract time series for each factor
        let time_series = Self::extract_time_series(assessments)?;
        
        // Calculate Pearson correlation for each pair
        for (factor1, series1) in &time_series {
            for (factor2, series2) in &time_series {
                if factor1 < factor2 {  // Avoid duplicates
                    let corr = Self::pearson_correlation(series1, series2);
                    correlations.push((
                        factor1.clone(),
                        factor2.clone(),
                        corr,
                    ));
                }
            }
        }
        
        Ok(correlations)
    }
    
    /// Pearson correlation coefficient
    fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
        assert_eq!(x.len(), y.len());
        let n = x.len() as f64;
        
        let mean_x = x.iter().sum::<f64>() / n;
        let mean_y = y.iter().sum::<f64>() / n;
        
        let cov: f64 = x.iter().zip(y.iter())
            .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
            .sum();
        
        let var_x: f64 = x.iter()
            .map(|xi| (xi - mean_x).powi(2))
            .sum();
        
        let var_y: f64 = y.iter()
            .map(|yi| (yi - mean_y).powi(2))
            .sum();
        
        cov / (var_x * var_y).sqrt()
    }
}
```

### 3.3 Bayesian Inference

Use the network to adjust risk scores based on observed evidence:

```rust
impl BayesianNetwork {
    /// Adjust risk score using Bayesian inference
    pub fn adjust_score(
        &self,
        base_score: f64,
        factors: &[RiskFactor],
        historical_context: &[Assessment],
    ) -> f64 {
        // 1. Set evidence in network
        for factor in factors {
            self.set_evidence(&factor.name, factor.value);
        }
        
        // 2. Perform belief propagation
        let beliefs = self.belief_propagation()?;
        
        // 3. Calculate posterior risk estimate
        let posterior_score = self.calculate_posterior(
            base_score,
            &beliefs,
            factors
        );
        
        // 4. Apply historical priors
        let adjusted_score = self.apply_historical_priors(
            posterior_score,
            historical_context
        );
        
        adjusted_score
    }
    
    /// Belief propagation algorithm (Junction Tree)
    fn belief_propagation(&self) -> Result<HashMap<String, Belief>> {
        // Convert to junction tree
        let junction_tree = self.to_junction_tree()?;
        
        // Initialize messages
        let mut messages = HashMap::new();
        
        // Message passing until convergence
        let mut converged = false;
        let mut iteration = 0;
        const MAX_ITERATIONS: usize = 100;
        const CONVERGENCE_THRESHOLD: f64 = 1e-6;
        
        while !converged && iteration < MAX_ITERATIONS {
            let old_messages = messages.clone();
            
            // Pass messages up and down tree
            junction_tree.pass_messages(&mut messages);
            
            // Check convergence
            converged = Self::messages_converged(
                &old_messages,
                &messages,
                CONVERGENCE_THRESHOLD
            );
            
            iteration += 1;
        }
        
        // Compute marginal beliefs
        let beliefs = junction_tree.compute_marginals(&messages)?;
        
        Ok(beliefs)
    }
    
    /// Calculate posterior score incorporating network beliefs
    fn calculate_posterior(
        &self,
        base_score: f64,
        beliefs: &HashMap<String, Belief>,
        factors: &[RiskFactor],
    ) -> f64 {
        let mut adjustment = 0.0;
        
        for factor in factors {
            if let Some(belief) = beliefs.get(&factor.name) {
                // Compare prior and posterior probabilities
                let prior_prob = self.get_prior_probability(&factor.name);
                let posterior_prob = belief.probability(factor.value);
                
                // Calculate adjustment based on difference
                let evidence_strength = posterior_prob - prior_prob;
                adjustment += evidence_strength * 0.1;  // Scale factor
            }
        }
        
        // Apply bounded adjustment
        (base_score + adjustment).max(0.0).min(1.0)
    }
}
```

### 3.4 Historical Prior Integration

Incorporate historical patterns as Bayesian priors:

```rust
impl BayesianNetwork {
    /// Apply historical priors to current assessment
    fn apply_historical_priors(
        &self,
        current_score: f64,
        historical: &[Assessment],
    ) -> f64 {
        if historical.is_empty() {
            return current_score;
        }
        
        // Calculate historical mean and variance
        let scores: Vec<f64> = historical
            .iter()
            .map(|a| a.risk_score.raw_score)
            .collect();
        
        let hist_mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let hist_var = scores.iter()
            .map(|s| (s - hist_mean).powi(2))
            .sum::<f64>() / scores.len() as f64;
        
        // Bayesian update: combine current observation with prior
        // Using conjugate prior (Normal-Normal model)
        
        // Prior precision (inverse variance)
        let prior_precision = 1.0 / hist_var;
        
        // Observation precision (based on confidence)
        let obs_precision = 10.0;  // Higher = trust current more
        
        // Posterior mean
        let posterior_mean = (
            prior_precision * hist_mean + obs_precision * current_score
        ) / (prior_precision + obs_precision);
        
        posterior_mean
    }
}
```

---

## 4. Time-Series Analysis

### 4.1 Trend Detection

Identify statistically significant trends in risk metrics:

```rust
pub struct TrendAnalyzer {
    significance_level: f64,  // e.g., 0.05 for 95% confidence
}

impl TrendAnalyzer {
    /// Mann-Kendall trend test
    pub fn mann_kendall_test(&self, data: &[f64]) -> TrendTestResult {
        let n = data.len();
        if n < 3 {
            return TrendTestResult::Insufficient;
        }
        
        // Calculate S statistic
        let mut s = 0i64;
        for i in 0..n {
            for j in (i+1)..n {
                s += Self::sign(data[j] - data[i]);
            }
        }
        
        // Calculate variance
        let var_s = (n * (n - 1) * (2 * n + 5)) as f64 / 18.0;
        
        // Calculate Z statistic
        let z = if s > 0 {
            (s as f64 - 1.0) / var_s.sqrt()
        } else if s < 0 {
            (s as f64 + 1.0) / var_s.sqrt()
        } else {
            0.0
        };
        
        // Test significance
        let critical_value = 1.96;  // For Î± = 0.05 (two-tailed)
        
        if z.abs() > critical_value {
            let direction = if z > 0 {
                TrendDirection::Increasing
            } else {
                TrendDirection::Decreasing
            };
            
            // Calculate Sen's slope
            let slope = self.sens_slope(data);
            
            TrendTestResult::Significant {
                direction,
                z_statistic: z,
                p_value: Self::z_to_p(z),
                slope,
            }
        } else {
            TrendTestResult::NoTrend {
                z_statistic: z,
                p_value: Self::z_to_p(z),
            }
        }
    }
    
    /// Sen's slope estimator
    fn sens_slope(&self, data: &[f64]) -> f64 {
        let n = data.len();
        let mut slopes = Vec::new();
        
        for i in 0..n {
            for j in (i+1)..n {
                let slope = (data[j] - data[i]) / (j - i) as f64;
                slopes.push(slope);
            }
        }
        
        // Return median slope
        slopes.sort_by(|a, b| a.partial_cmp(b).unwrap());
        slopes[slopes.len() / 2]
    }
    
    fn sign(x: f64) -> i64 {
        if x > 0.0 { 1 }
        else if x < 0.0 { -1 }
        else { 0 }
    }
    
    fn z_to_p(z: f64) -> f64 {
        // Convert Z to two-tailed p-value using error function
        use statrs::function::erf::erfc;
        erfc(z.abs() / std::f64::consts::SQRT_2)
    }
}

pub enum TrendTestResult {
    Significant {
        direction: TrendDirection,
        z_statistic: f64,
        p_value: f64,
        slope: f64,
    },
    NoTrend {
        z_statistic: f64,
        p_value: f64,
    },
    Insufficient,  // Not enough data
}

pub enum TrendDirection {
    Increasing,
    Decreasing,
}
```

### 4.2 Change Point Detection

Identify abrupt shifts in risk levels:

```rust
impl TrendAnalyzer {
    /// CUSUM change point detection
    pub fn detect_change_points(&self, data: &[f64]) -> Vec<ChangePoint> {
        let n = data.len();
        if n < 10 {
            return Vec::new();
        }
        
        // Calculate mean and standard deviation
        let mean = data.iter().sum::<f64>() / n as f64;
        let std_dev = (
            data.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / n as f64
        ).sqrt();
        
        // CUSUM parameters
        let k = 0.5 * std_dev;  // Allowance
        let h = 4.0 * std_dev;  // Decision interval
        
        let mut cusum_pos = 0.0;
        let mut cusum_neg = 0.0;
        let mut change_points = Vec::new();
        
        for (i, &value) in data.iter().enumerate() {
            // Update CUSUM statistics
            cusum_pos = (cusum_pos + value - mean - k).max(0.0);
            cusum_neg = (cusum_neg - value + mean - k).max(0.0);
            
            // Check for change points
            if cusum_pos > h {
                change_points.push(ChangePoint {
                    index: i,
                    direction: ChangeDirection::Increase,
                    magnitude: cusum_pos,
                    confidence: self.calculate_confidence(cusum_pos, h),
                });
                cusum_pos = 0.0;
            }
            
            if cusum_neg > h {
                change_points.push(ChangePoint {
                    index: i,
                    direction: ChangeDirection::Decrease,
                    magnitude: cusum_neg,
                    confidence: self.calculate_confidence(cusum_neg, h),
                });
                cusum_neg = 0.0;
            }
        }
        
        change_points
    }
}

pub struct ChangePoint {
    pub index: usize,
    pub direction: ChangeDirection,
    pub magnitude: f64,
    pub confidence: f64,
}
```

### 4.3 Seasonal Decomposition

Separate trend, seasonal, and residual components:

```rust
impl TrendAnalyzer {
    /// STL (Seasonal and Trend decomposition using Loess)
    pub fn seasonal_decomposition(
        &self,
        data: &[f64],
        period: usize,
    ) -> Decomposition {
        // 1. Detrend the data using loess smoothing
        let trend = self.loess_smooth(data, 0.3);
        
        let detrended: Vec<f64> = data.iter()
            .zip(&trend)
            .map(|(d, t)| d - t)
            .collect();
        
        // 2. Extract seasonal component
        let seasonal = self.extract_seasonal(&detrended, period);
        
        // 3. Calculate residuals
        let residual: Vec<f64> = data.iter()
            .zip(&trend)
            .zip(&seasonal)
            .map(|((d, t), s)| d - t - s)
            .collect();
        
        Decomposition {
            trend,
            seasonal,
            residual,
            period,
        }
    }
    
    /// LOESS (Locally Estimated Scatterplot Smoothing)
    fn loess_smooth(&self, data: &[f64], bandwidth: f64) -> Vec<f64> {
        let n = data.len();
        let window_size = ((n as f64 * bandwidth).ceil() as usize).max(3);
        let mut smoothed = Vec::with_capacity(n);
        
        for i in 0..n {
            // Determine window
            let start = i.saturating_sub(window_size / 2);
            let end = (i + window_size / 2).min(n);
            
            // Weighted least squares regression
            let (a, b) = self.weighted_regression(data, i, start, end);
            
            // Predict smoothed value
            smoothed.push(a + b * i as f64);
        }
        
        smoothed
    }
}
```

---

## 5. Monte Carlo Simulation

### 5.1 Simulation Framework

```rust
pub struct MonteCarloSimulator {
    rng: StdRng,
    num_iterations: usize,
    scenario_generator: ScenarioGenerator,
    escalation_model: EscalationModel,
}

impl MonteCarloSimulator {
    /// Run Monte Carlo simulation
    pub fn simulate(
        &mut self,
        initial_state: &WorldState,
        time_horizon: Duration,
    ) -> SimulationResults {
        let mut outcomes = Vec::with_capacity(self.num_iterations);
        
        for _ in 0..self.num_iterations {
            let outcome = self.simulate_single_trajectory(
                initial_state.clone(),
                time_horizon
            );
            outcomes.push(outcome);
        }
        
        self.analyze_outcomes(outcomes)
    }
    
    fn simulate_single_trajectory(
        &mut self,
        mut state: WorldState,
        time_horizon: Duration,
    ) -> Outcome {
        let mut elapsed = Duration::ZERO;
        let mut events = Vec::new();
        
        while elapsed < time_horizon && !state.is_terminal() {
            // Sample next event from probability distributions
            let event = self.sample_next_event(&state);
            
            // Apply event to state
            state.apply_event(&event);
            events.push(event.clone());
            
            elapsed += event.time_delta;
            
            // Check for nuclear war
            if state.nuclear_war_occurred {
                break;
            }
        }
        
        Outcome {
            final_state: state.clone(),
            events,
            nuclear_war: state.nuclear_war_occurred,
            escalation_level: state.escalation_level,
            time_to_war: if state.nuclear_war_occurred {
                Some(elapsed)
            } else {
                None
            },
        }
    }
}
```

### 5.2 Event Sampling

```rust
impl MonteCarloSimulator {
    /// Sample next event from probability distribution
    fn sample_next_event(&mut self, state: &WorldState) -> Event {
        // Calculate probability of each event type
        let event_probs = self.calculate_event_probabilities(state);
        
        // Sample event type using weighted random choice
        let event_type = self.weighted_sample(&event_probs);
        
        // Generate specific event of that type
        let event = self.generate_event(event_type, state);
        
        event
    }
    
    fn calculate_event_probabilities(
        &self,
        state: &WorldState
    ) -> HashMap<EventType, f64> {
        let mut probs = HashMap::new();
        
        // Base rates for different event types
        probs.insert(EventType::MilitaryIncident, 0.001);  // Per day
        probs.insert(EventType::NuclearThreat, 0.0005);
        probs.insert(EventType::AirspaceViolation, 0.0002);
        probs.insert(EventType::TechnicalFailure, 0.0001);
        probs.insert(EventType::DiplomaticCrisis, 0.0003);
        
        // Adjust based on current state
        self.adjust_probabilities(&mut probs, state);
        
        // Normalize
        let total: f64 = probs.values().sum();
        for prob in probs.values_mut() {
            *prob /= total;
        }
        
        probs
    }
    
    fn adjust_probabilities(
        &self,
        probs: &mut HashMap<EventType, f64>,
        state: &WorldState
    ) {
        // Escalation multiplier based on current tensions
        let escalation_mult = 1.0 + state.tension_level * 5.0;
        
        // Adjust all probabilities
        for prob in probs.values_mut() {
            *prob *= escalation_mult;
        }
        
        // Specific adjustments
        if state.active_conflicts.len() > 0 {
            *probs.get_mut(&EventType::MilitaryIncident).unwrap() *= 3.0;
        }
        
        if state.communication_channels_degraded {
            *probs.get_mut(&EventType::TechnicalFailure).unwrap() *= 2.0;
        }
    }
}
```

### 5.3 Results Analysis

```rust
impl MonteCarloSimulator {
    fn analyze_outcomes(&self, outcomes: Vec<Outcome>) -> SimulationResults {
        let n = outcomes.len() as f64;
        
        // Calculate probabilities
        let nuclear_war_count = outcomes.iter()
            .filter(|o| o.nuclear_war)
            .count() as f64;
        let nuclear_war_prob = nuclear_war_count / n;
        
        // Time to war statistics (for cases where war occurred)
        let war_times: Vec<Duration> = outcomes.iter()
            .filter_map(|o| o.time_to_war)
            .collect();
        
        let mean_time_to_war = if !war_times.is_empty() {
            war_times.iter().map(|d| d.as_secs_f64()).sum::<f64>() 
                / war_times.len() as f64
        } else {
            f64::INFINITY
        };
        
        // Confidence intervals (Wilson score interval)
        let (prob_lower, prob_upper) = self.wilson_score_interval(
            nuclear_war_count,
            n,
            0.95
        );
        
        // Escalation level distribution
        let escalation_dist = self.calculate_escalation_distribution(&outcomes);
        
        SimulationResults {
            num_simulations: outcomes.len(),
            nuclear_war_probability: nuclear_war_prob,
            probability_confidence_interval: (prob_lower, prob_upper),
            mean_time_to_war_days: mean_time_to_war / 86400.0,
            escalation_distribution: escalation_dist,
            most_common_pathway: self.identify_common_pathway(&outcomes),
            critical_events: self.identify_critical_events(&outcomes),
        }
    }
    
    /// Wilson score interval for binomial proportion
    fn wilson_score_interval(
        &self,
        successes: f64,
        n: f64,
        confidence: f64
    ) -> (f64, f64) {
        let z = match confidence {
            0.90 => 1.645,
            0.95 => 1.96,
            0.99 => 2.576,
            _ => 1.96,
        };
        
        let p = successes / n;
        let denominator = 1.0 + z * z / n;
        let center = (p + z * z / (2.0 * n)) / denominator;
        let margin = (z / denominator) * ((p * (1.0 - p) / n) + (z * z / (4.0 * n * n))).sqrt();
        
        (
            (center - margin).max(0.0),
            (center + margin).min(1.0)
        )
    }
}
```

---

## 6. Uncertainty Quantification

### 6.1 Sources of Uncertainty

```rust
pub enum UncertaintySource {
    /// Measurement uncertainty in data
    Measurement {
        variable: String,
        std_error: f64,
    },
    
    /// Model uncertainty (choice of model)
    Model {
        alternatives: Vec<String>,
        model_weights: Vec<f64>,
    },
    
    /// Parameter uncertainty (model parameters)
    Parameter {
        parameter: String,
        distribution: Distribution,
    },
    
    /// Aleatory uncertainty (inherent randomness)
    Aleatory {
        process: String,
        variance: f64,
    },
    
    /// Epistemic uncertainty (lack of knowledge)
    Epistemic {
        knowledge_gap: String,
        confidence_range: (f64, f64),
    },
}
```

### 6.2 Uncertainty Propagation

```rust
pub struct UncertaintyPropagator {
    num_samples: usize,
}

impl UncertaintyPropagator {
    /// Propagate uncertainties through calculation
    pub fn propagate(
        &self,
        inputs: &[UncertainInput],
        calculation: impl Fn(&[f64]) -> f64,
    ) -> UncertainOutput {
        let mut rng = StdRng::from_entropy();
        let mut results = Vec::with_capacity(self.num_samples);
        
        for _ in 0..self.num_samples {
            // Sample from input distributions
            let samples: Vec<f64> = inputs.iter()
                .map(|input| input.sample(&mut rng))
                .collect();
            
            // Calculate result
            let result = calculation(&samples);
            results.push(result);
        }
        
        // Analyze results
        self.analyze_results(results)
    }
    
    fn analyze_results(&self, mut results: Vec<f64>) -> UncertainOutput {
        results.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let mean = results.iter().sum::<f64>() / results.len() as f64;
        let variance = results.iter()
            .map(|r| (r - mean).powi(2))
            .sum::<f64>() / results.len() as f64;
        
        // Percentiles for confidence intervals
        let p_2_5 = results[(results.len() as f64 * 0.025) as usize];
        let p_97_5 = results[(results.len() as f64 * 0.975) as usize];
        
        UncertainOutput {
            mean,
            median: results[results.len() / 2],
            std_dev: variance.sqrt(),
            confidence_interval_95: (p_2_5, p_97_5),
            distribution: results,
        }
    }
}
```

---

## 7. Historical Calibration

### 7.1 Calibration Against Known Events

```rust
pub struct HistoricalCalibrator {
    known_events: Vec<HistoricalEvent>,
}

pub struct HistoricalEvent {
    pub name: String,
    pub date: DateTime<Utc>,
    pub actual_risk_level: f64,  // Expert assessment
    pub factors_present: Vec<RiskFactor>,
    pub outcome: EventOutcome,
}

impl HistoricalCalibrator {
    /// Validate model against historical events
    pub fn calibrate(&self, model: &RiskModel) -> CalibrationResults {
        let mut predictions = Vec::new();
        let mut actuals = Vec::new();
        
        for event in &self.known_events {
            // Calculate what model would have predicted
            let prediction = model.calculate_risk(&event.factors_present);
            
            predictions.push(prediction);
            actuals.push(event.actual_risk_level);
        }
        
        // Calculate calibration metrics
        let correlation = self.calculate_correlation(&predictions, &actuals);
        let rmse = self.calculate_rmse(&predictions, &actuals);
        let calibration_curve = self.calculate_calibration_curve(
            &predictions,
            &actuals
        );
        
        CalibrationResults {
            correlation,
            rmse,
            calibration_curve,
            is_well_calibrated: correlation > 0.7 && rmse < 0.15,
        }
    }
    
    fn calculate_calibration_curve(
        &self,
        predictions: &[f64],
        actuals: &[f64]
    ) -> Vec<(f64, f64)> {
        // Bin predictions and calculate actual frequencies
        const NUM_BINS: usize = 10;
        let mut bins = vec![Vec::new(); NUM_BINS];
        
        for (pred, actual) in predictions.iter().zip(actuals) {
            let bin = ((pred * NUM_BINS as f64).floor() as usize)
                .min(NUM_BINS - 1);
            bins[bin].push(*actual);
        }
        
        // Calculate mean actual value per bin
        bins.iter()
            .enumerate()
            .map(|(i, bin_actuals)| {
                let predicted_prob = (i as f64 + 0.5) / NUM_BINS as f64;
                let actual_freq = if bin_actuals.is_empty() {
                    predicted_prob  // No data
                } else {
                    bin_actuals.iter().sum::<f64>() / bin_actuals.len() as f64
                };
                (predicted_prob, actual_freq)
            })
            .collect()
    }
}
```

### 7.2 Backtesting Framework

```rust
pub struct BacktestEngine {
    historical_assessments: Vec<Assessment>,
}

impl BacktestEngine {
    /// Test model performance over historical period
    pub fn backtest(
        &self,
        model: &RiskModel,
        test_period: (DateTime<Utc>, DateTime<Utc>)
    ) -> BacktestResults {
        let mut predictions = Vec::new();
        let mut actuals = Vec::new();
        
        // Walk forward through history
        for i in 1..self.historical_assessments.len() {
            let current = &self.historical_assessments[i];
            
            if current.timestamp < test_period.0 
                || current.timestamp > test_period.1 {
                continue;
            }
            
            // Use data up to this point
            let training_data = &self.historical_assessments[0..i];
            
            // Make prediction
            let prediction = model.predict_next(training_data);
            
            // Record actual value
            let actual = current.risk_score.raw_score;
            
            predictions.push(prediction);
            actuals.push(actual);
        }
        
        // Calculate performance metrics
        BacktestResults {
            correlation: self.calculate_correlation(&predictions, &actuals),
            mae: self.calculate_mae(&predictions, &actuals),
            rmse: self.calculate_rmse(&predictions, &actuals),
            direction_accuracy: self.calculate_direction_accuracy(
                &predictions,
                &actuals
            ),
        }
    }
}
```

---

## 8. Implementation Specifications

### 8.1 Complete Risk Calculation Pipeline

```rust
pub struct RiskCalculationPipeline {
    weighted_scorer: WeightedScorer,
    bayesian_network: BayesianNetwork,
    monte_carlo: MonteCarloSimulator,
    trend_analyzer: TrendAnalyzer,
    uncertainty_propagator: UncertaintyPropagator,
}

impl RiskCalculationPipeline {
    /// Execute complete risk calculation
    pub fn calculate_comprehensive_risk(
        &mut self,
        factors: &[RiskFactor],
        historical_data: &[Assessment],
    ) -> ComprehensiveRiskScore {
        // 1. Base weighted score
        let base_score = self.weighted_scorer.calculate_base_score(factors);
        
        // 2. Bayesian adjustment
        let adjusted_score = self.bayesian_network.adjust_score(
            base_score,
            factors,
            historical_data
        );
        
        // 3. Trend analysis
        let trend = self.trend_analyzer.analyze_trend(historical_data);
        
        // 4. Monte Carlo simulation
        let simulation = self.monte_carlo.simulate(
            &WorldState::from_factors(factors),
            Duration::from_days(365)
        );
        
        // 5. Uncertainty quantification
        let uncertainty = self.uncertainty_propagator.propagate(
            &factors.iter().map(|f| f.to_uncertain_input()).collect::<Vec<_>>(),
            |inputs| self.calculate_risk_from_inputs(inputs)
        );
        
        ComprehensiveRiskScore {
            seconds_to_midnight: self.score_to_seconds(adjusted_score),
            raw_score: adjusted_score,
            base_score,
            confidence_interval: uncertainty.confidence_interval_95,
            trend,
            simulation_results: simulation,
            primary_drivers: self.identify_primary_drivers(factors),
            uncertainty_analysis: uncertainty,
        }
    }
    
    /// Convert risk score to Doomsday Clock seconds
    fn score_to_seconds(&self, score: f64) -> u32 {
        // Map [0, 1] to [0, 1440] (midnight to noon)
        // Higher score = closer to midnight
        let seconds = ((1.0 - score) * 1440.0) as u32;
        seconds.clamp(0, 1440)
    }
}
```

### 8.2 Performance Requirements

| Operation | Target | Method |
|-----------|--------|--------|
| Factor evaluation | <10ms per factor | Efficient algorithms, caching |
| Base score calculation | <100ms | Optimized weighted sum |
| Bayesian inference | <1s | Efficient belief propagation |
| Trend analysis | <500ms | Pre-computed correlations |
| Monte Carlo (10K iter) | <60s | Parallel execution |
| Complete assessment | <5min | Pipeline optimization |

### 8.3 Validation Requirements

1. **Unit Tests**: Every calculation function must have unit tests
2. **Property Tests**: Key algorithms validated with property-based testing
3. **Historical Validation**: Correlation >0.7 with expert assessments
4. **Calibration**: Predicted probabilities match observed frequencies
5. **Sensitivity**: Results stable under reasonable parameter variations

---

## Conclusion

This methodology provides a rigorous, scientifically defensible framework for nuclear war risk assessment. By combining multiple analytical techniquesâ€”weighted scoring, Bayesian inference, time-series analysis, and Monte Carlo simulationâ€”the system produces robust estimates with proper uncertainty quantification.

The approach is transparent, auditable, and grounded in both statistical theory and expert judgment. Historical calibration ensures predictions align with real-world events, while ongoing validation maintains accuracy over time.

**Key Strengths:**
- Multi-method triangulation reduces single-model bias
- Uncertainty quantification provides honest confidence estimates
- Historical validation ensures real-world accuracy
- Transparent calculations enable scrutiny and improvement

**Next Steps:**
1. Implement core algorithms in Rust
2. Validate against historical events (Cuban Missile Crisis, etc.)
3. Calibrate weights through expert elicitation
4. Build comprehensive test suite
5. Establish continuous validation process

*"The only winning move is not to play."*
