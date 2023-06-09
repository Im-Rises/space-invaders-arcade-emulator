import React, {useEffect} from 'react';
import init, {run} from 'si-emu-pkg';
import './SiEmulator.scss';
import PropTypes from 'prop-types';
import {backgroundVersionList} from '../constants/screen-constants';

const SiEmulator = props => {
	const siScreenWidth = 224;
	const siScreenHeight = 256;

	const backgroundImageRef = React.useRef(null);
	const siGameCanvasRef = React.useRef(null);
	const screenPanelRef = React.useRef(null);

	const handleResize = () => {
		const screenPanel = screenPanelRef.current;
		const screenPanelWidth = screenPanel.clientWidth;
		const screenPanelHeight = screenPanel.clientHeight;
		const screenPanelAspectRatio = screenPanelWidth / screenPanelHeight;

		const backgroundImage = backgroundImageRef.current;
		const backgroundImageWidth = backgroundImage.clientWidth;
		const backgroundImageHeight = backgroundImage.clientHeight;
		const backgroundImageAspectRatio = backgroundImageWidth / backgroundImageHeight;

		if (screenPanelAspectRatio > backgroundImageAspectRatio) {
			backgroundImageRef.current.style.width = `${screenPanelHeight * backgroundImageAspectRatio}px`;
			backgroundImageRef.current.style.height = `${screenPanelHeight}px`;
			siGameCanvasRef.current.style.height = `${screenPanelHeight}px`;
		} else {
			backgroundImageRef.current.style.width = `${screenPanelWidth}px`;
			backgroundImageRef.current.style.height = `${screenPanelWidth / backgroundImageAspectRatio}px`;
			siGameCanvasRef.current.style.height = `${screenPanelWidth / backgroundImageAspectRatio}px`;
		}
	};

	useEffect(() => {
		init().then(() => {
			// run(props.canvasId, props.screenMode, props.oneAdditional, props.twoAdditional, props.earlyUfo, props.coinDemo, props.romDataH, props.romDataG, props.romDataF, props.romDataE);
			run(props.canvasId, props.screenMode, props.oneAdditional, props.twoAdditional, props.earlyUfo, props.coinDemo);// Debug
		});

		window.addEventListener('resize', handleResize);

		return () => {
			window.removeEventListener('resize', handleResize);
		};
	}, []);

	return (
		<div className={'si-emulator'}>
			<div ref={screenPanelRef} className={'screen-panel'}>
				<img ref={backgroundImageRef} onLoad={handleResize}
					className={'background-image'}
					src={backgroundVersionList[props.backgroundVersion].image} alt={''}/>
				<canvas ref={siGameCanvasRef} id={props.canvasId} width={siScreenWidth} height={siScreenHeight}/>
			</div>
			<div className={'control-panel'}>
				<button onClick={() => console.log('sqdqsd')} id={'si-button-up'}>Up</button>
				<button id={'si-button-left'}>Left</button>
				<button id={'si-button-right'}>Right</button>
				<button id={'si-button-coin'}>Coin</button>
				<button id={'si-button-1p'}>1P</button>
				<button id={'si-button-2p'}>2P</button>
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
