import { bindActionCreators } from 'redux';
import { connect } from 'react-redux';
import Signup from '../components/Signup/Signup';
import * as SignupActions from '../actions/signup';

function mapStateToProps(state) {
  return {
    signup: 0
  };
}

function mapDispatchToProps(dispatch) {
  return bindActionCreators(SignupActions, dispatch);
}

export default connect(mapStateToProps, mapDispatchToProps)(Signup);
