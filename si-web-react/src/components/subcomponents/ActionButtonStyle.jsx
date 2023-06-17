import React from 'react';
import PropTypes from 'prop-types';
import './ActionButtonStyle.scss';

export const ActionButtonStyle = props => (
	<button id={props.id} className={'no-select video-game-button'}>{props.label}</button>
);

ActionButtonStyle.propTypes = {
	id: PropTypes.string.isRequired,
	label: PropTypes.string.isRequired,
};
