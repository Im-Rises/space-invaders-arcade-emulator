import React, {useEffect, useState} from 'react';
import {InputFile} from './components/InputFile';
import './App.scss';

import init, {run} from './si-emu-pkg';

let romLoaded = 0;

const App = () => {
	const canvasId = 'canvas';
	const [romDataH, setRomDataH] = useState(null);
	const [romDataG, setRomDataG] = useState(null);
	const [romDataF, setRomDataF] = useState(null);
	const [romDataE, setRomDataE] = useState(null);

	const onFileLoad = (() => () => {
		romLoaded++;
		console.log('romLoaded', romLoaded);
		// if (romLoaded < 4) {
		// 	return;
		// }

		// console.log('run');
		//
		// if (romDataE === null) {
		// 	console.log('romDataE is null');
		// 	return;
		// }

		// // Print the roms data
		// console.log('romDataH', romDataH);
		// console.log('romDataG', romDataG);
		// console.log('romDataF', romDataF);
		// console.log('romDataE', romDataE);

		// run(canvasId, romDataH, romDataG, romDataF, romDataE);
	})();

	useEffect(() => {
		init().then(() => {
			console.log('init done');
		});
	}, []);

	return (
		<div className='App'>
			<div>
				<p>Load ROM H</p>
				<InputFile setRomData={setRomDataH} onFileLoad={() => onFileLoad()}/>
				<p>Load ROM G</p>
				<InputFile setRomData={setRomDataG} onFileLoad={() => onFileLoad()}/>
				<p>Load ROM F</p>
				<InputFile setRomData={setRomDataF} onFileLoad={() => onFileLoad()}/>
				<p>Load ROM E</p>
				<InputFile setRomData={setRomDataE} onFileLoad={() => onFileLoad()}/>
				<button onClick={() => {
					if (!romDataH || !romDataG || !romDataF || !romDataE) {
						console.log('romDataH', romDataH);
						console.log('romDataG', romDataG);
						console.log('romDataF', romDataF);
						console.log('romDataE', romDataE);
						return;
					}

					run(canvasId, romDataH, romDataG, romDataF, romDataE);
				}
				}>Run
				</button>
			</div>
			<div>
				<div>
					<canvas id={canvasId}/>
					<img id={'si-background'} alt={'background'}/>
				</div>
				<div>
					<button id={'button-left'}/>
					<button id={'button-right'}/>
					<button id={'button-up'}/>
					<button id={'button-player1-start'}/>
					<button id={'button-player2-start'}/>
					<button id={'button-coin'}/>
				</div>
			</div>
		</div>
	);
};

export default App;
