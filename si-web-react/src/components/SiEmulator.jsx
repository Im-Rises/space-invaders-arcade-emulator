import React, {useEffect} from 'react';
import init, {run} from 'si-emu-pkg';

const SiEmulator = props => {
	console.log('SiEmulator props', props);
	useEffect(() => {
		init().then(() => {
			console.log('init done');
			// run(props.canvasId, props.screenMode, props.oneAdditional, props.twoAdditional, props.earlyUfo, props.coinDemo, props.romDataH, props.romDataG, props.romDataF, props.romDataE);
			// run(props.canvasId, props.screenMode, props.oneAdditional, props.twoAdditional, props.earlyUfo, props.coinDemo);
			console.log('run done');
		});
	}, []);

	return (
		<>
			<img src={'./assets/si-emu.png'} alt={'Si BG'}/>
			<canvas id={props.canvasId} width={224} height={256}/>
		</>
	);
};

export default SiEmulator;
