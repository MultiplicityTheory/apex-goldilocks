use std::collections::BTreeMap;

pub const INDEX_SIZE: i32 = 12288;
pub const NUM_PAGES: i32 = 48;
pub const BYTES_PER_PAGE: i32 = 256;
pub const NUM_BELTS: i32 = 1;

pub fn index_map(page: i32, belt: i32, offset: i32, encode_belt: bool) -> Result<i32, String> {
    if !(0..NUM_PAGES).contains(&page) {
        return Err(format!(
            "page must be in [0, {}], got {}",
            NUM_PAGES - 1,
            page
        ));
    }
    if !(0..BYTES_PER_PAGE).contains(&offset) {
        return Err(format!(
            "offset must be in [0, {}], got {}",
            BYTES_PER_PAGE - 1,
            offset
        ));
    }
    if encode_belt {
        if !(0..NUM_BELTS).contains(&belt) {
            return Err(format!(
                "belt must be in [0, {}], got {}",
                NUM_BELTS - 1,
                belt
            ));
        }
        Ok(page * (BYTES_PER_PAGE * NUM_BELTS) + belt * BYTES_PER_PAGE + offset)
    } else {
        Ok(page * BYTES_PER_PAGE + offset)
    }
}

pub fn inverse_index_map(index: i32, encode_belt: bool) -> Result<(i32, i32, i32), String> {
    if encode_belt {
        let max_index = NUM_PAGES * BYTES_PER_PAGE * NUM_BELTS - 1;
        if !(0..=max_index).contains(&index) {
            return Err(format!("index must be in [0, {}], got {}", max_index, index));
        }
        let page = index / (BYTES_PER_PAGE * NUM_BELTS);
        let remainder = index % (BYTES_PER_PAGE * NUM_BELTS);
        let belt = remainder / BYTES_PER_PAGE;
        let offset = remainder % BYTES_PER_PAGE;
        Ok((page, belt, offset))
    } else {
        if !(0..INDEX_SIZE).contains(&index) {
            return Err(format!(
                "index must be in [0, {}], got {}",
                INDEX_SIZE - 1,
                index
            ));
        }
        let page = index / BYTES_PER_PAGE;
        let offset = index % BYTES_PER_PAGE;
        let belt = 0;
        Ok((page, belt, offset))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexAddress {
    pub page: i32,
    pub belt: i32,
    pub offset: i32,
    pub linear_index: i32,
}

pub fn make_address(
    page: i32,
    belt: i32,
    offset: i32,
    encode_belt: bool,
) -> Result<IndexAddress, String> {
    let linear_index = index_map(page, belt, offset, encode_belt)?;
    Ok(IndexAddress {
        page,
        belt,
        offset,
        linear_index,
    })
}

pub fn decode_address(index: i32, encode_belt: bool) -> Result<IndexAddress, String> {
    let (page, belt, offset) = inverse_index_map(index, encode_belt)?;
    Ok(IndexAddress {
        page,
        belt,
        offset,
        linear_index: index,
    })
}

pub fn prime_factors(mut n: i32) -> BTreeMap<i32, i32> {
    let mut factors = BTreeMap::new();
    if n <= 1 {
        return factors;
    }
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            *factors.entry(d).or_insert(0) += 1;
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        *factors.entry(n).or_insert(0) += 1;
    }
    factors
}

pub fn check_projector_admissibility(p: i32, r: i32, n: i32) -> bool {
    if p <= 1 || r <= 0 {
        return false;
    }
    if let Some(pow) = p.checked_pow(r as u32) {
        n % pow == 0
    } else {
        false
    }
}

pub fn max_admissible_level(p: i32, n: i32) -> i32 {
    let factors = prime_factors(n);
    factors.get(&p).copied().unwrap_or(0)
}

pub fn list_admissible_projectors(n: i32, max_prime: i32) -> Vec<(i32, i32)> {
    let factors = prime_factors(n);
    let mut admissible = Vec::new();
    for (&p, &max_r) in &factors {
        if p <= max_prime {
            for r in 1..=max_r {
                admissible.push((p, r));
            }
        }
    }
    admissible.sort();
    admissible
}

pub fn prime_policy_default(_d: i32) -> i32 {
    3
}

pub fn level_policy_default(_d: i32) -> i32 {
    1
}

pub fn perm_policy_default(_d: i32) -> Vec<i32> {
    Vec::new()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidationResults {
    pub index_size: i32,
    pub index_size_check: bool,
    pub pages: i32,
    pub bytes_per_page: i32,
    pub factorization: BTreeMap<i32, i32>,
    pub admissible_projectors: Vec<(i32, i32)>,
    pub policies_status: String,
    pub size_consistency: bool,
}

pub fn validate_phase0_constraints() -> ValidationResults {
    let factors = prime_factors(INDEX_SIZE);
    let admissible = list_admissible_projectors(INDEX_SIZE, 20);
    ValidationResults {
        index_size: INDEX_SIZE,
        index_size_check: INDEX_SIZE == 12288,
        pages: NUM_PAGES,
        bytes_per_page: BYTES_PER_PAGE,
        factorization: factors,
        admissible_projectors: admissible,
        policies_status: "UNPROVEN".to_string(),
        size_consistency: INDEX_SIZE == NUM_PAGES * BYTES_PER_PAGE,
    }
}

pub fn print_phase0_summary() {
    println!("{}", "=".repeat(70));
    println!("Phase 0 — Hard Constraints Summary");
    println!("{}", "=".repeat(70));
    println!("Index size n = {} (fixed)", INDEX_SIZE);
    println!("Pages: {}", NUM_PAGES);
    println!("Bytes per page: {}", BYTES_PER_PAGE);
    println!("Formula: {} × {} = {}", NUM_PAGES, BYTES_PER_PAGE, INDEX_SIZE);
    println!();

    let factors = prime_factors(INDEX_SIZE);
    print!("Prime factorization: {} = ", INDEX_SIZE);
    let factor_parts: Vec<String> = factors
        .iter()
        .map(|(&p, &e)| format!("{}^{}", p, e))
        .collect();
    println!("{}", factor_parts.join(" × "));
    println!();

    println!("Admissible projectors (p^r | n, p ≤ 20):");
    for (p, r) in list_admissible_projectors(INDEX_SIZE, 20) {
        println!("  (p={}, r={}): {}^{} = {}", p, r, p, r, p.pow(r as u32));
    }
    println!();

    println!("Policies (status: UNPROVEN):");
    println!("  primePolicy(d) → p (default: p=3)");
    println!("  levelPolicy(d) → r (default: r=1)");
    println!("  permPolicy(d) → π (default: identity)");
    println!();
    println!("⚠️  All policies are UNPROVEN until mathematically fixed and verified.");
    println!("{}", "=".repeat(70));
}
