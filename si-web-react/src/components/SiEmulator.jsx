import React, {useEffect} from 'react';
import init, {run} from 'si-emu-pkg';
import './SiEmulator.scss';
import PropTypes from 'prop-types';
import {backgroundVersionList} from '../constants/screen-constants';

const SiEmulator = props => {
	const siScreenWidth = 224;
	const siScreenHeight = 256;

	useEffect(() => {
		init().then(() => {
			// run(props.canvasId, props.screenMode, props.oneAdditional, props.twoAdditional, props.earlyUfo, props.coinDemo, props.romDataH, props.romDataG, props.romDataF, props.romDataE);
			run(props.canvasId, props.screenMode, props.oneAdditional, props.twoAdditional, props.earlyUfo, props.coinDemo);// Debug
		});
	}, []);
	return (
		<div className={'si-emulator'}>
			<div className={'screen-panel'}>
				<img
					className={'background-image'}
					src={backgroundVersionList.find(backgroundVersion => backgroundVersion.value === props.backgroundVersion).image}
					alt={'background-img'}/>
				<canvas id={props.canvasId} width={siScreenWidth} height={siScreenHeight}/>
			</div>
			<div className={'control-panel'}>
				<button id={'si-button-up'} className={'no-select'}>Up</button>
				<button id={'si-button-left'} className={'no-select'}>Left</button>
				<button id={'si-button-right'} className={'no-select'}>Right</button>
				<button id={'si-button-coin'} className={'no-select'}>Coin</button>
				<button id={'si-button-1p'} className={'no-select'}>1P</button>
				<button id={'si-button-2p'} className={'no-select'}>2P</button>
			</div>
		</div>
	);
};

SiEmulator.propTypes = {
	canvasId: PropTypes.string.isRequired,
	screenMode: PropTypes.string.isRequired,
	backgroundVersion: PropTypes.string.isRequired,
	oneAdditional: PropTypes.bool.isRequired,
	twoAdditional: PropTypes.bool.isRequired,
	earlyUfo: PropTypes.bool.isRequired,
	coinDemo: PropTypes.bool.isRequired,
	romDataH: PropTypes.object.isRequired,
	romDataG: PropTypes.object.isRequired,
	romDataF: PropTypes.object.isRequired,
	romDataE: PropTypes.object.isRequired,
};

export default SiEmulator;
