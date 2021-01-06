use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::DispatchError;
#[test]
fn create_claim_test() {
    new_test_ext().execute_with(|| {
        // create claim oK
        let proof = "bace".as_bytes().to_vec();
        assert_ok!(POE::create_claim(Origin::signed(1), proof.clone()));
        
        //create claim Error  ProofAlreadyClaimed
        assert_noop!(
			POE::create_claim(Origin::signed(1), proof.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);

        //create claim proof size too large
        let proof2 = "dafdsafdsaf13".as_bytes();
        assert_noop!(POE::create_claim(Origin::signed(1), proof),
        Error::<Test>::ProofAlreadyClaimed);
    });

}


#[test]
fn revoke_claim_test() {
    new_test_ext().execute_with(|| {
        // revoke claim oK
        let proof = "bace".as_bytes().to_vec();
        assert_ok!(POE::create_claim(Origin::signed(1), proof.clone()));
        assert_ok!(POE::revoke_claim(Origin::signed(1), proof.clone()));
        
        //revoke claim Error  NoSuchProof
        let proof1 = "bacea".as_bytes().to_vec();
        assert_noop!(POE::revoke_claim(Origin::signed(2), proof1),
        Error::<Test>::NoSuchProof);

        //revoke claim Error NotProofOwner
        let proof2 = "baceaa".as_bytes().to_vec();
        assert_ok!(POE::create_claim(Origin::signed(2), proof2.clone()));
        assert_noop!(POE::revoke_claim(Origin::signed(3), proof2),
        Error::<Test>::NotProofOwner);
    });

}

#[test]
fn move_claim_test() {
    new_test_ext().execute_with(|| {
        // move claim oK
        let proof = "bace".as_bytes().to_vec();
        assert_ok!(POE::create_claim(Origin::signed(1), proof.clone()));
        assert_ok!(POE::move_claim(Origin::signed(1), proof, 99));
        
        //move claim Error  NoSuchProof
        let proof1 = "bacea".as_bytes().to_vec();
        assert_noop!(POE::move_claim(Origin::signed(2), proof1, 3),
        Error::<Test>::NoSuchProof);

        //move claim Error NotProofOwner
        let proof2 = "baceaa".as_bytes().to_vec();
        assert_ok!(POE::create_claim(Origin::signed(2), proof2.clone()));
        assert_noop!(POE::move_claim(Origin::signed(3), proof2, 4),
        Error::<Test>::NotProofOwner);
    });
}