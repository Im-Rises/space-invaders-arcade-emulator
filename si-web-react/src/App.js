import React, {useEffect, useState} from 'react';
import {InputFile} from './components/InputFile';
import backgroundImage from './images/invaders.png';
import './App.scss';

import init, {run} from 'si-emu-pkg';

const App = () => {
	const canvasId = 'si-canvas-id';
	const [romDataH, setRomDataH] = useState(null);
	const [romDataG, setRomDataG] = useState(null);
	const [romDataF, setRomDataF] = useState(null);
	const [romDataE, setRomDataE] = useState(null);
	const [romsLoaded, setRomsLoaded] = useState(false);
	const selectVersionRef = React.useRef(null);

	useEffect(() => {
		init().then(() => {
			console.log('init done');
		});
	}, []);

	return (
		<>
		</>
	);
};

export default App;
