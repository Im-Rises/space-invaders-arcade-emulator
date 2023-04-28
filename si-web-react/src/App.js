import React from 'react';
import './App.scss';
import {InputFile} from './components/InputFile';
// eslint-disable-next-line no-unused-vars
import init, {run} from '../si-emu-pkg/space_invaders_arcade_emulator';

const App = async () => {
	await init();
	// run();
	return (

		<div className='App'>
			<div>
				<InputFile/>
				<InputFile/>
				<InputFile/>
				<InputFile/>
			</div>
			<div>
				<div>
					<canvas id='canvas'/>
					<img id='si-background' alt={'background'}/>
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
