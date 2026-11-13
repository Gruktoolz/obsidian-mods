fn kotost_fp_eat(p: &mut Poses, n: &str) {
    if n == "right_arm" {
        crate::kotost_fp::eat_arm(p);
    }
}
