use bdk_coin_select::{
    Candidate, ChangePolicy, CoinSelector, Drain, DrainWeights, Target, TargetFee, TargetOutputs,
};

#[test]
/// A change policy having a min value of 0 can lead to a drain output
/// with non-zero weight and zero value.
fn drain_with_zero_value_is_possible() {
    let mut target = Target {
        fee: TargetFee::default(),
        outputs: TargetOutputs {
            value_sum: 99_000,
            weight_sum: 200,
            n_outputs: 1,
        },
    };

    let candidates = vec![Candidate {
        value: 100_000,
        weight: 100,
        input_count: 1,
        is_segwit: true,
    }];

    let drain_weights = DrainWeights {
        output_weight: 100,
        spend_weight: 1_000,
        n_outputs: 1,
    };

    let mut cs = CoinSelector::new(&candidates);
    cs.select(0);
    assert!(cs.is_target_met(target));

    let with_change_excess = cs.excess(
        target,
        Drain {
            value: 0,
            weights: drain_weights,
        },
    );
    assert!(with_change_excess > 0);

    // Add the excess to the target output so that drain value is 0.
    target.outputs.value_sum += with_change_excess as u64;
    assert!(cs.is_target_met(target));

    // Set change policy with min value 0.
    let change_policy = ChangePolicy {
        min_value: 0,
        drain_weights,
    };
    let drain = cs.drain(target, change_policy);
    // Selection has a drain with zero value.
    assert!(drain.is_some());
    assert_eq!(drain.value, 0);

    // Increasing min value will remove the drain.
    let change_policy = ChangePolicy {
        min_value: 1,
        drain_weights,
    };
    assert!(cs.drain(target, change_policy).is_none());
}
