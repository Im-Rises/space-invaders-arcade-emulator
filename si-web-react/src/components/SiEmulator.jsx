import React, {useEffect} from 'react';
import init, {run} from 'si-emu-pkg';
import './SiEmulator.scss';
import SIBackground from '../images/background/invaders.png';
import PropTypes from 'prop-types';

const SiEmulator = props => {
	const siScreenWidth = 224;
	const siScreenHeight = 256;

	useEffect(() => {
		init().then(() => {
			// run(props.canvasId, props.screenMode, props.oneAdditional, props.twoAdditional, props.earlyUfo, props.coinDemo, props.romDataH, props.romDataG, props.romDataF, props.romDataE);
			run(props.canvasId, props.screenMode, props.oneAdditional, props.twoAdditional, props.earlyUfo, props.coinDemo);
		});
	}, []);

	return (
		<div className={'si-emulator'}>
			<div className={'screen-panel'}>
				<canvas id={props.canvasId} width={siScreenWidth} height={siScreenHeight}
					style={{
						backgroundColor: props.isBackgroundVisible ? 'transparent' : 'black',
						backgroundImage: `url(${props.isBackgroundVisible ? SIBackground : ''})`,
						backgroundSize: 'contain',
						backgroundPosition: 'center',
						backgroundRepeat: 'no-repeat',
						objectFit: 'fit-content',
					}}
				/>
			</div>
			<div className={'control-panel'}>
				<button>Up</button>
				<button>Left</button>
				<button>Right</button>
				<button>Coin</button>
				<button>Start</button>
				<button>1P</button>
				<button>2P</button>
			</div>
		</div>
	);
};

SiEmulator.propTypes = {
	canvasId: PropTypes.string.isRequired,
	screenMode: PropTypes.string.isRequired,
	isBackgroundVisible: PropTypes.bool.isRequired,
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
