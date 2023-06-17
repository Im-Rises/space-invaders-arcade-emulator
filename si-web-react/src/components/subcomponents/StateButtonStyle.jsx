import React from 'react';
import PropTypes from 'prop-types';
import './StateButtonStyle.scss';

export const StateButtonStyle = props => (
	<button id={props.id} className={'no-select start-btn'}>{props.label}</button>
);

StateButtonStyle.propTypes = {
	id: PropTypes.string.isRequired,
	label: PropTypes.string.isRequired,
};
