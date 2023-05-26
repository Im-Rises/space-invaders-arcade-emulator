import React, {useEffect, useState} from 'react';
import {InputFile} from './components/InputFile';
import backgroundImage from './images/invaders.png';
import './App.scss';

import init, {run} from 'si-emu-pkg';

const App = () => {
	const canvasId = 'canvas';
	const [romDataH, setRomDataH] = useState(null);
	const [romDataG, setRomDataG] = useState(null);
	const [romDataF, setRomDataF] = useState(null);
	const [romDataE, setRomDataE] = useState(null);
	const [romsLoaded, setRomsLoaded] = useState(false);

	useEffect(() => {
		init().then(() => {
			console.log('init done');
		});
	}, []);

	return (
		<>
			<div className={'panel-game'}>
				<div className={'canvas-si-panel'}>
					<canvas id={canvasId} className={'canvas-si'}/>
					<img src={backgroundImage} alt={'background'}/>
				</div>
				<div className={'control-panel'}>
					{/* <button id={'button-left'}/> */}
					{/* <button id={'button-right'}/> */}
					{/* <button id={'button-up'}/> */}
					{/* <button id={'button-player1-start'}/> */}
					{/* <button id={'button-player2-start'}/> */}
					{/* <button id={'button-coin'}/> */}
				</div>
			</div>
			{
				!romsLoaded
				&& (
					<div className={'panel-rom-loader'}>
						<p>Load ROM H</p>
						<InputFile setRomData={setRomDataH}/>
						<p>Load ROM G</p>
						<InputFile setRomData={setRomDataG}/>
						<p>Load ROM F</p>
						<InputFile setRomData={setRomDataF}/>
						<p>Load ROM E</p>
						<InputFile setRomData={setRomDataE}/>
						<button onClick={() => {
							if (!romDataH || !romDataG || !romDataF || !romDataE) {
								console.log('All roms loaded');
								return;
							}

							setRomsLoaded(true);
							run(canvasId, romDataH, romDataG, romDataF, romDataE);
						}
						}>Run
						</button>
					</div>
				)
			}

		</>
	);
};

export default App;
