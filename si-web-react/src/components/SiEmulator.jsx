import React, {useEffect} from 'react';
import init, {run} from 'si-emu-pkg';
import PropTypes from 'prop-types';
import {backgroundVersionList} from '../constants/screen-constants';
import {ActionButtonStyle} from './subcomponents/ActionButtonStyle';
import {StateButtonStyle} from './subcomponents/StateButtonStyle';
import './SiEmulator.scss';

const SiEmulator = props => {
	const canvasId = 'si-canvas-id';
	const buttonPrefix = 'si-button';
	const siScreenWidth = 224;
	const siScreenHeight = 256;

	useEffect(() => {
		init().then(() => {
			// run(canvasId, buttonPrefix, props.screenMode, props.oneAdditional, props.twoAdditional, props.earlyUfo, props.coinDemo, props.romDataH, props.romDataG, props.romDataF, props.romDataE);
			run(canvasId, buttonPrefix, props.screenMode, props.oneAdditional, props.twoAdditional, props.earlyUfo, props.coinDemo);// Debug
		});
	}, []);
	return (
		<div className={'si-emulator'}>
			<div className={'screen-panel'}>
				<img
					className={'background-image'}
					src={backgroundVersionList.find(backgroundVersion => backgroundVersion.value === props.backgroundVersion).image}
					alt={'background-img'}/>
				<canvas id={canvasId} width={siScreenWidth} height={siScreenHeight}/>
			</div>
			<div className={'control-panel'}>
				<div>
					<StateButtonStyle id={buttonPrefix + '-1p'} label={'1P'}
						className={'no-select'}>1P</StateButtonStyle>
					<ActionButtonStyle id={buttonPrefix + '-coin'} label={'$'}
						className={'no-select'}>Coin</ActionButtonStyle>
					<StateButtonStyle id={buttonPrefix + '-2p'} label={'2P'}
						className={'no-select'}>2P</StateButtonStyle>
				</div>
				<div>
					<ActionButtonStyle id={buttonPrefix + '-left'} label={'←'}
						className={'no-select'}>Left</ActionButtonStyle>
					<ActionButtonStyle id={buttonPrefix + '-up'} label={'↑'}
						className={'no-select'}>Up</ActionButtonStyle>
					<ActionButtonStyle id={buttonPrefix + '-right'} label={'→'}
						className={'no-select'}>Right</ActionButtonStyle>
				</div>
			</div>
		</div>
	);
};

SiEmulator.propTypes = {
	screenMode: PropTypes.string.isRequired,
	backgroundVersion: PropTypes.string.isRequired,
	oneAdditional: PropTypes.bool.isRequired,
	twoAdditional: PropTypes.bool.isRequired,
	earlyUfo: PropTypes.bool.isRequired,
	coinDemo: PropTypes.bool.isRequired,
	// romDataH: PropTypes.object.isRequired,
	// romDataG: PropTypes.object.isRequired,
	// romDataF: PropTypes.object.isRequired,
	// romDataE: PropTypes.object.isRequired,
};

export default SiEmulator;
