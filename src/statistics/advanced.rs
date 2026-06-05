use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
    },
    statistics::{advanced, stats},
};

#[derive(Clone, Default)]
pub struct Advanced;

impl Advanced {
    pub fn hurst_exponent(&self, time_series_values: &[f64]) -> StockTrekResult<f64> {
        advanced::hurst_exponent(time_series_values)
    }
    pub fn mutual_information(
        &self,
        first_series: &[f64],
        second_series: &[f64],
    ) -> StockTrekResult<f64> {
        advanced::mutual_information(first_series, second_series)
    }
    pub fn sample_entropy(
        &self,
        time_series_values: &[f64],
        embedding_dimension: usize,
        tolerance: f64,
    ) -> StockTrekResult<f64> {
        advanced::sample_entropy(time_series_values, embedding_dimension, tolerance)
    }
    pub fn shannon_entropy(&self, probability_distribution: &[f64]) -> StockTrekResult<f64> {
        advanced::shannon_entropy(probability_distribution)
    }
}

pub fn hurst_exponent(time_series_values: &[f64]) -> StockTrekResult<f64> {
    let n = time_series_values.len();
    if n < 20 {
        return Err(StockTrekError::Stats(StatsError::InvalidParameters {
            function: "hurst_exponent",
            message: format!("time series has {} elements, need at least 20", n),
        }));
    }
    let mean = stats::mean(time_series_values)?;
    let mut cumulative = Vec::with_capacity(n);
    let mut sum = 0.0;
    for &x in time_series_values {
        sum += x - mean;
        cumulative.push(sum);
    }
    let min = cumulative.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = cumulative.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;
    let std = stats::standard_deviation(time_series_values, 0)?;
    if std == 0.0 {
        return Err(StockTrekError::Stats(StatsError::DivisionByZero {
            function: "hurst_exponent",
            detail: "standard deviation is zero".to_string(),
        }));
    }
    let rs = range / std;
    Ok(rs.ln() / (n as f64).ln())
}

pub fn mutual_information(first_series: &[f64], second_series: &[f64]) -> StockTrekResult<f64> {
    if first_series.len() != second_series.len() {
        return Err(StockTrekError::Stats(StatsError::MismatchedLengths {
            function: "mutual_information",
            first_len: first_series.len(),
            second_len: second_series.len(),
        }));
    }
    if first_series.is_empty() {
        return Err(StockTrekError::Stats(StatsError::EmptyInput {
            function: "mutual_information",
        }));
    }
    let n = first_series.len();
    // simple binning approach
    let bins = (n as f64).sqrt() as usize + 1;
    let min_x = first_series.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_x = first_series
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let min_y = second_series.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_y = second_series
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let width_x = (max_x - min_x) / bins as f64;
    let width_y = (max_y - min_y) / bins as f64;
    if width_x == 0.0 || width_y == 0.0 {
        return Err(StockTrekError::Stats(StatsError::DivisionByZero {
            function: "mutual_information",
            detail: "bin width is zero; all data points are identical in one or both dimensions"
                .to_string(),
        }));
    }
    let mut joint = vec![vec![0.0; bins]; bins];
    let mut px = vec![0.0; bins];
    let mut py = vec![0.0; bins];
    for i in 0..n {
        let xi = ((first_series[i] - min_x) / width_x).floor() as usize;
        let yi = ((second_series[i] - min_y) / width_y).floor() as usize;
        let xi = xi.min(bins - 1);
        let yi = yi.min(bins - 1);
        joint[xi][yi] += 1.0;
        px[xi] += 1.0;
        py[yi] += 1.0;
    }
    let n_f = n as f64;
    let mut mi = 0.0;
    for i in 0..bins {
        for j in 0..bins {
            let pxy = joint[i][j] / n_f;
            if pxy > 0.0 {
                let px_i = px[i] / n_f;
                let py_j = py[j] / n_f;
                mi += pxy * (pxy / (px_i * py_j)).ln();
            }
        }
    }
    Ok(mi)
}

pub fn sample_entropy(
    time_series_values: &[f64],
    embedding_dimension: usize,
    tolerance: f64,
) -> StockTrekResult<f64> {
    let n = time_series_values.len();
    if n <= embedding_dimension + 1 || tolerance <= 0.0 {
        return Err(StockTrekError::Stats(StatsError::InvalidParameters {
            function: "sample_entropy",
            message: format!(
                "n={}, embedding_dimension={}, tolerance={}: need n > embedding_dimension + 1 and tolerance > 0",
                n, embedding_dimension, tolerance
            ),
        }));
    }
    let mut count_m = 0.0;
    let mut count_m1 = 0.0;
    // i can only go up to n - embedding_dimension - 1 to leave room for the m+1 check
    for i in 0..(n - embedding_dimension) {
        // j must go up to n - embedding_dimension to include all valid m-vectors
        for j in (i + 1)..(n - embedding_dimension + 1) {
            // Check if m-dimensional vectors match (Chebyshev distance)
            let mut match_m = true;
            for k in 0..embedding_dimension {
                if (time_series_values[i + k] - time_series_values[j + k]).abs() > tolerance {
                    match_m = false;
                    break;
                }
            }
            if !match_m {
                continue;
            }
            count_m += 1.0;
            // Only check the (m+1)th dimension if j also has room for it
            if j < n - embedding_dimension
                && (time_series_values[i + embedding_dimension]
                    - time_series_values[j + embedding_dimension])
                    .abs()
                    <= tolerance
            {
                count_m1 += 1.0;
            }
        }
    }
    if count_m == 0.0 || count_m1 == 0.0 {
        return Err(StockTrekError::Stats(StatsError::DivisionByZero {
            function: "sample_entropy",
            detail: "no matching vector pairs found".to_string(),
        }));
    }
    Ok(-f64::ln(count_m1 / count_m))
}

pub fn shannon_entropy(probability_distribution: &[f64]) -> StockTrekResult<f64> {
    if probability_distribution.is_empty() {
        return Err(StockTrekError::Stats(StatsError::EmptyInput {
            function: "shannon_entropy",
        }));
    }
    let mut entropy = 0.0;
    for &p in probability_distribution {
        if p < 0.0 {
            return Err(StockTrekError::Stats(StatsError::InvalidParameters {
                function: "shannon_entropy",
                message: format!("negative probability {}", p),
            }));
        }
        if p > 0.0 {
            entropy -= p * p.ln();
        }
    }
    Ok(entropy)
}
