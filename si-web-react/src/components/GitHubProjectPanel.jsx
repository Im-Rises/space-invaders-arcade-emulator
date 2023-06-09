import React from 'react';
import './GitHubProjectPanel.scss';
import GitHubLogoImage from '../images/logo/github.svg';
import PropTypes from 'prop-types';

const GitHubProjectPanel = props => (
	<a href={props.link} target={'_blank'} rel='noreferrer'>
		<div className='Project-Panel'>
			<img src={GitHubLogoImage} alt={'GithubLogoImage'}/>
			<p>{props.author}</p>
		</div>
	</a>
);

GitHubProjectPanel.propTypes = {
	link: PropTypes.string.isRequired,
	author: PropTypes.string.isRequired,
};

export default GitHubProjectPanel;
