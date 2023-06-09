import React from 'react';
import {InputFile} from './InputFile';
import {useState} from 'react';
import SiEmulator from './SiEmulator';
import GitHubProjectPanel from './GitHubProjectPanel';
import './SiGamePanel.scss';
import {AUTHOR, REPO} from '../settings/github-constants';
// import DemoVideoMode1 from '../images/demo/si-demo-1-sv.png';
import DemoVideoMode2 from '../images/demo/si-demo-2-tv.png';
// import DemoVideoMode3 from '../images/demo/si-demo-3-cv.png';
// import BlackBackground from '../images/background/bg_black.png';
// import SIBackground1 from '../images/background/bg_invaders_1.png';
import SIBackground2 from '../images/background/bg_invaders_2.png';
import SelectorButton from './SelectorButton';

const SiGamePanel = () => {
	const [isRomLoaded, setRomsLoaded] = React.useState(false);
	const canvasId = 'si-canvas-id';
	// const screenModeRef = React.useRef(null);
	const oneAdditionalCheckboxRef = React.useRef(null);
	const twoAdditionalCheckboxRef = React.useRef(null);
	const earlyUfoCheckboxRef = React.useRef(null);
	const coinDemoCheckboxRef = React.useRef(null);
	const [romDataH, setRomDataH] = useState(null);
	const [romDataG, setRomDataG] = useState(null);
	const [romDataF, setRomDataF] = useState(null);
	const [romDataE, setRomDataE] = useState(null);
	// const backgroundVersionRef = React.useRef(null);

	const imgDemoRef = React.useRef(null);
	const imgBackgroundRef = React.useRef(null);

	const screenModeList = [{
		value: 'SV',
		label: 'Black and white (SV)',
	}, {
		value: 'TV',
		label: 'Original (TV)',
	}, {
		value: 'CV',
		label: 'Colored (CV)',
	}];

	const backgroundVersionList = [{
		value: 'hidden',
		label: 'Hidden',
	}, {
		value: '1',
		label: 'Version 1',
	}, {
		value: '2',
		label: 'Version 2',
	}];

	return (
		<>
			{
				!isRomLoaded ? (
					<>
						<GitHubProjectPanel link={REPO} author={AUTHOR}/>
						<div className={'setup-panel'}>
							<div className={'title'}>
								<h1>Space Invaders Arcade Emulator</h1>
							</div>
							<div className={'screen-mode-panel'}>
								<div className={'img-demo'}>
									<img src={DemoVideoMode2} alt={'demo-game-screen'} className={'img-demo'}
										ref={imgDemoRef}/>
									<img src={SIBackground2} alt={'demo-game-screen'} className={'img-bg'}
										ref={imgBackgroundRef}/>
								</div>
								<div className={'control-mode'}>
									<div>
										<SelectorButton elementList={screenModeList}
											// defaultValue={screenModeList[2].value}
										/>
									</div>
									<div>
										<SelectorButton elementList={backgroundVersionList}
											// defaultValue={backgroundVersionList[2].value}
										/>
									</div>
									<div>
										<label><input type={'checkbox'} ref={oneAdditionalCheckboxRef}/>
                                                One additional life</label>
										<label><input type={'checkbox'} ref={twoAdditionalCheckboxRef}/>
                                                Two additional lives</label>
										<label><input type={'checkbox'} ref={earlyUfoCheckboxRef}/>
                                                UFO at 1000 points</label>
										<label><input type={'checkbox'} ref={coinDemoCheckboxRef}/>
                                                Coin in demo</label>
									</div>
								</div>
							</div>
							<div className={'load-rom-panel'}>
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
										// return;
									}

									setRomsLoaded(true);
								}
								}>Run
								</button>
							</div>
						</div>
					</>
				)
					: (
						<>
							<SiEmulator
								canvasId={canvasId}
								// screenMode={screenModeRef.current.value}
								// backgroundVersion={backgroundVersionRef.current.value}
								oneAdditional={oneAdditionalCheckboxRef.current.checked}
								twoAdditional={twoAdditionalCheckboxRef.current.checked}
								earlyUfo={earlyUfoCheckboxRef.current.checked}
								coinDemo={coinDemoCheckboxRef.current.checked}
								romDataH={romDataH}
								romDataG={romDataG}
								romDataF={romDataF}
								romDataE={romDataE}
							/>
							<button onClick={() => {
								setRomsLoaded(false);
							}}>Back
							</button>
						</>
					)
			}
		</>
	);
};

export default SiGamePanel;
