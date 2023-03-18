
// struct Stark {
//     field: Field,
//     expansion_factor: usize,
//     num_colinearity_checks: usize,
//     security_level: usize,
//     num_randomizers: usize,
//     num_registers: usize,
//     original_trace_length: usize,
//     generator: BigUint,
//     omega: BigUint,
//     omicron: BigUint,
//     omicron_domain: Vec<BigUint>,
//     fri: Fri,
// }

struct Stark {
    field: FielElement,
    expansaion_factor: usize,
    num_colinearity_checks: usize,
    security_level: usize,
    num_randomizers: usize,
    num_registers: usize,
    original_trace_length: usize,
    generator: BigUint,
    omega: BigUint,
    omicron: BigUint,
    omicron_domain: Vec<BigUint>,
    fri: Fri,
}


impl Stark {
    fn new(
        field: FieldElement,
        expansion_factor: usize,
        num_colinearity_checks: usize,
        security_level: usize,
        num_registers: usize,
        num_cycles: usize,
        transition_contraints_degree: usize,
    ) -> Self {
        assert!(
            (BigUint::one() << (field.p.bits() - 2)) >= BigUint::from(security_level),
                "p must have at least as many bits as security level"
        );
        assert!(
            expansion_factor.bitand(expansion_factor -1) == 0,
            "expansion factor must be a power of 2"
        );
        assert!(
            expansion_factor >= 4,
            "expansion factor must be 4 or greater"
        );
        assert!(
            num_colinearity_checks*2 >= security_level,
            "number of colinearity checks must be at least half of security level"
        );
        // Generate data for constructing Stark struct
        let num_randomizers = 4*num_colinearity_checks;
        let original_trace_length = num_cycles;

        let randomized_trace_length = original_trace_length + num_randomizers;
        let omicron_domain_length = 1 << ((randomized_trace_length * transition_contraints_degree).next_power_of_two());
        let fri_domain_length = omicron_domain_length * expansion_factor;

        let generator = field.generator();
        let omega = field.primitive_nth_root(fri_domain_length);
        let omicron_domain = (0..omicron_domain_length).map(|i| omicron.clone().pow(&[i as u32])).collect::<Vec<BigUint>>();
        let fri = Fri::new(generator.clone(), omega.clone(), fri_domain_length, expansion_factor, num_colinearity_checks);

        Stark {
            field,
            expansion_factor,
            num_colinearity_checks,
            security_level,
            num_randomizers,
            num_registers,
            original_trace_length,
            generator,
            omega,
            omicron,
            omicron_domain,
            fri,
        }

    }
}


