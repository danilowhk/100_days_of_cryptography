
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

    fn transition_degree_bounds(&self, transition_constraints: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
        let point_degrees = std::iter::once(1).chain(std::iter::repeat(self.original_trace_length + self.num_randomizers -1).take(2 * self.num_registers)).collect::<Vec<usize>>();

        transition_contraints.iter().map(|a| { a.iter().map(|k_v| k_v.iter().map(|(r, l)| r*l).sum::<usize>()).max().unwrap()}).collect()
    }

    fn transition_quotient_degree_bounds(&self, transition_constraints: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
        let degree_bounds = self.transition_degree_bounds(transition_constraints);
        degree_bounds.iter().map(|d| d - (self.original_trace_length -1)).collect()
    }

    fn max_degree(&self, transition_constraints: &Vec<Vec<(usize, usize)>>) -> usize {
        let md = self.transition_quotient_degree_bounds(transition_constraints).iter().max().unwrap();
        (1 << (md.next_power_of_tow().trailing_zeros() as usize)) -1
    }

}


