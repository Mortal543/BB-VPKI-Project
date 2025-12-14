use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub timestamp: String,
    pub certificate_issuance_rate: f64,
    pub revocation_latency_ms: f64,
    pub authentication_delay_us: f64,
    pub authentication_p50_us: f64,
    pub authentication_p95_us: f64,
    pub authentication_p99_us: f64,
    pub message_signing_time_us: f64,
    pub message_verification_time_us: f64,
    pub cache_hit_rate: f64,
    pub cache_miss_rate: f64,
    pub consensus_latency_ms: f64,
    pub consensus_p50_ms: f64,
    pub consensus_p95_ms: f64,
    pub consensus_p99_ms: f64,
    pub blockchain_tps: f64,
    pub blockchain_size_mb: f64,
    pub pruned_blocks: usize,
    pub deprecated_count: usize,
    pub system_uptime_secs: u64,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339(),
            certificate_issuance_rate: 0.0,
            revocation_latency_ms: 0.0,
            authentication_delay_us: 0.0,
            authentication_p50_us: 0.0,
            authentication_p95_us: 0.0,
            authentication_p99_us: 0.0,
            message_signing_time_us: 0.0,
            message_verification_time_us: 0.0,
            cache_hit_rate: 0.0,
            cache_miss_rate: 0.0,
            consensus_latency_ms: 0.0,
            consensus_p50_ms: 0.0,
            consensus_p95_ms: 0.0,
            consensus_p99_ms: 0.0,
            blockchain_tps: 0.0,
            blockchain_size_mb: 0.0,
            pruned_blocks: 0,
            deprecated_count: 0,
            system_uptime_secs: 0,
        }
    }

    pub fn print_report(&self) {
        println!("\n╔═══════════════════════════════════════════════════════╗");
        println!("║       BB-VPKI Performance Evaluation Report          ║");
        println!("╠═══════════════════════════════════════════════════════╣");
        println!("║ Timestamp: {:<42} ║", self.timestamp);
        println!("║                                                       ║");
        println!("║ 1. Certificate Issuance Rate                         ║");
        println!(
            "║    → {:<46.2} certs/sec ║",
            self.certificate_issuance_rate
        );
        println!("║                                                       ║");
        println!("║ 2. Certificate Revocation Latency                    ║");
        println!("║    → {:<46.2} ms ║", self.revocation_latency_ms);
        println!("║                                                       ║");
        println!("║ 3. Authentication Delay                               ║");
        println!("║    → Avg: {:<40.2} μs ║", self.authentication_delay_us);
        println!("║    → p50: {:<41.2} μs ║", self.authentication_p50_us);
        println!("║    → p95: {:<41.2} μs ║", self.authentication_p95_us);
        println!("║    → p99: {:<41.2} μs ║", self.authentication_p99_us);
        println!("║    → Target: <3000 μs (<3ms) for safety-critical apps ║");
        println!("║                                                       ║");
        println!("║ 4. Message Signing Time                               ║");
        println!("║    → {:<46.2} μs ║", self.message_signing_time_us);
        println!("║                                                       ║");
        println!("║ 5. Message Verification Time                          ║");
        println!("║    → {:<46.2} μs ║", self.message_verification_time_us);
        println!("║                                                       ║");
        println!("║ 6. Edge Node Cache Hit Rate                           ║");
        println!("║    → {:<46.2}% ║", self.cache_hit_rate);
        println!("║    → Miss Rate: {:<42.2}% ║", self.cache_miss_rate);
        println!("║                                                       ║");
        println!("║ 7. Blockchain Transaction Throughput                  ║");
        println!("║    → {:<46.2} TPS ║", self.blockchain_tps);
        println!("║                                                       ║");
        println!("║ 9. Consensus Latency                                  ║");
        println!("║    → Avg: {:<40.2} ms ║", self.consensus_latency_ms);
        println!("║    → p50: {:<41.2} ms ║", self.consensus_p50_ms);
        println!("║    → p95: {:<41.2} ms ║", self.consensus_p95_ms);
        println!("║    → p99: {:<41.2} ms ║", self.consensus_p99_ms);
        println!("║                                                       ║");
        println!("║ 8. Blockchain Storage Management                      ║");
        println!("║    → Size: {:<43.2} MB ║", self.blockchain_size_mb);
        println!("║    → Pruned blocks: {:<34} ║", self.pruned_blocks);
        println!("║                                                       ║");
        println!("║ System Uptime: {:<38} sec ║", self.system_uptime_secs);
        println!("║ Deprecated Certificates: {:<26} ║", self.deprecated_count);
        println!("╚═══════════════════════════════════════════════════════╝\n");
    }

    pub fn save_to_csv(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_exists = std::path::Path::new(filename).exists();
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;

        let mut wtr = csv::Writer::from_writer(file);

        if !file_exists {
            wtr.write_record(&[
                "timestamp",
                "certificate_issuance_rate_certs_per_sec",
                "revocation_latency_ms",
                "authentication_delay_us",
                "authentication_p50_us",
                "authentication_p95_us",
                "authentication_p99_us",
                "message_signing_time_us",
                "message_verification_time_us",
                "cache_hit_rate_percent",
                "cache_miss_rate_percent",
                "consensus_latency_ms",
                "consensus_p50_ms",
                "consensus_p95_ms",
                "consensus_p99_ms",
                "blockchain_tps",
                "blockchain_size_mb",
                "pruned_blocks",
                "deprecated_certificates",
                "system_uptime_secs",
            ])?;
        }

        wtr.write_record(&[
            &self.timestamp,
            &self.certificate_issuance_rate.to_string(),
            &self.revocation_latency_ms.to_string(),
            &self.authentication_delay_us.to_string(),
            &self.authentication_p50_us.to_string(),
            &self.authentication_p95_us.to_string(),
            &self.authentication_p99_us.to_string(),
            &self.message_signing_time_us.to_string(),
            &self.message_verification_time_us.to_string(),
            &self.cache_hit_rate.to_string(),
            &self.cache_miss_rate.to_string(),
            &self.consensus_latency_ms.to_string(),
            &self.consensus_p50_ms.to_string(),
            &self.consensus_p95_ms.to_string(),
            &self.consensus_p99_ms.to_string(),
            &self.blockchain_tps.to_string(),
            &self.blockchain_size_mb.to_string(),
            &self.pruned_blocks.to_string(),
            &self.deprecated_count.to_string(),
            &self.system_uptime_secs.to_string(),
        ])?;

        wtr.flush()?;
        Ok(())
    }

    pub fn save_detailed_csv(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(filename)?;

        writeln!(file, "BB-VPKI Performance Metrics Report")?;
        writeln!(file, "Timestamp,{}", self.timestamp)?;
        writeln!(file, "")?;
        writeln!(file, "Metric,Value,Unit,Target,Status")?;
        writeln!(
            file,
            "Certificate Issuance Rate,{:.2},certs/sec,>1000,{}",
            self.certificate_issuance_rate,
            if self.certificate_issuance_rate > 1000.0 {
                "PASS"
            } else {
                "FAIL"
            }
        )?;
        writeln!(
            file,
            "Revocation Latency,{:.2},ms,<100,{}",
            self.revocation_latency_ms,
            if self.revocation_latency_ms < 100.0 {
                "PASS"
            } else {
                "FAIL"
            }
        )?;
        writeln!(
            file,
            "Authentication Delay,{:.2},μs,<3000,{}",
            self.authentication_delay_us,
            if self.authentication_delay_us < 3000.0 {
                "PASS"
            } else {
                "FAIL"
            }
        )?;
        writeln!(
            file,
            "Authentication Delay p50,{:.2},μs,N/A,INFO",
            self.authentication_p50_us
        )?;
        writeln!(
            file,
            "Authentication Delay p95,{:.2},μs,N/A,INFO",
            self.authentication_p95_us
        )?;
        writeln!(
            file,
            "Authentication Delay p99,{:.2},μs,N/A,INFO",
            self.authentication_p99_us
        )?;
        writeln!(
            file,
            "Message Signing Time,{:.2},μs,<100,{}",
            self.message_signing_time_us,
            if self.message_signing_time_us < 100.0 {
                "PASS"
            } else {
                "FAIL"
            }
        )?;
        writeln!(
            file,
            "Message Verification Time,{:.2},μs,<100,{}",
            self.message_verification_time_us,
            if self.message_verification_time_us < 100.0 {
                "PASS"
            } else {
                "FAIL"
            }
        )?;
        writeln!(
            file,
            "Cache Hit Rate,{:.2},%,>85,{}",
            self.cache_hit_rate,
            if self.cache_hit_rate > 85.0 {
                "PASS"
            } else {
                "FAIL"
            }
        )?;
        writeln!(
            file,
            "Blockchain TPS,{:.2},transactions/sec,>100,{}",
            self.blockchain_tps,
            if self.blockchain_tps > 100.0 {
                "PASS"
            } else {
                "FAIL"
            }
        )?;
        writeln!(
            file,
            "Consensus Latency,{:.2},ms,N/A,INFO",
            self.consensus_latency_ms
        )?;
        writeln!(
            file,
            "Consensus Latency p50,{:.2},ms,N/A,INFO",
            self.consensus_p50_ms
        )?;
        writeln!(
            file,
            "Consensus Latency p95,{:.2},ms,N/A,INFO",
            self.consensus_p95_ms
        )?;
        writeln!(
            file,
            "Consensus Latency p99,{:.2},ms,N/A,INFO",
            self.consensus_p99_ms
        )?;
        writeln!(
            file,
            "Blockchain Size,{:.2},MB,N/A,INFO",
            self.blockchain_size_mb
        )?;
        writeln!(file, "Pruned Blocks,{},blocks,N/A,INFO", self.pruned_blocks)?;
        writeln!(
            file,
            "Deprecated Certificates,{},count,N/A,INFO",
            self.deprecated_count
        )?;
        writeln!(
            file,
            "System Uptime,{},seconds,N/A,INFO",
            self.system_uptime_secs
        )?;

        Ok(())
    }
}
