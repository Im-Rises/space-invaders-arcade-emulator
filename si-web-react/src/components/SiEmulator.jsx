import React, {useEffect} from 'react';
import init, {run} from 'si-emu-pkg';

export const SiEmulator = () => {
	useEffect(() => {
		init().then(() => {
			console.log('init done');
			// run(canvasId, selectVersionRef.current.value, romDataH, romDataG, romDataF, romDataE);
			// run(canvasId, selectVersionRef.current.value);// Debug
			console.log('run done');
		});
	}, []);

	return (
		<>
			{/* <canvas id={'canvas'} width={224} height={256}/> */}
		</>
	);
};
