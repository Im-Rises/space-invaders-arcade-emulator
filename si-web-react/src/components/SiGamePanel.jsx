import React from 'react';
import {InputFile} from './InputFile';
import {useState} from 'react';
import SiEmulator from './SiEmulator';
import GitHubProjectPanel from './GitHubProjectPanel';
import './SiGamePanel.scss';
import {AUTHOR, REPO} from '../settings/github-constants';
import DemoVideoMode1 from '../images/demo/si-demo-1-sv.png';
import DemoVideoMode2 from '../images/demo/si-demo-2-tv.png';
import DemoVideoMode3 from '../images/demo/si-demo-3-cv.png';

const SiGamePanel = () => {
	const [isRomLoaded, setRomsLoaded] = React.useState(false);
	const canvasId = 'si-canvas-id';
	const screenModeRef = React.useRef(null);
	const oneAdditionalCheckboxRef = React.useRef(null);
	const twoAdditionalCheckboxRef = React.useRef(null);
	const earlyUfoCheckboxRef = React.useRef(null);
	const coinDemoCheckboxRef = React.useRef(null);
	const [romDataH, setRomDataH] = useState(null);
	const [romDataG, setRomDataG] = useState(null);
	const [romDataF, setRomDataF] = useState(null);
	const [romDataE, setRomDataE] = useState(null);
	const backgroundVersionRef = React.useRef(null);

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
								<img src={DemoVideoMode1} alt={'background_1'}/>
								<div className={'control-mode'}>
									<select id={'screen-mode'} defaultValue={'CV'} ref={screenModeRef}>
										<option value={'SV'}>Black and white (SV)</option>
										<option value={'TV'}>Original (TV)</option>
										<option value={'CV'}>Colored (CV)</option>
									</select>
									<select id={'background'} ref={backgroundVersionRef} defaultValue={'background_1'}>
										<option value={'hidden'}>Hidden</option>
										<option value={'background_1'}>Version 1</option>
										<option value={'background_2'}>Version 2</option>
									</select>
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
						<SiEmulator
							canvasId={canvasId}
							screenMode={screenModeRef.current.value}
							backgroundVersion={backgroundVersionRef.current.value}
							oneAdditional={oneAdditionalCheckboxRef.current.checked}
							twoAdditional={twoAdditionalCheckboxRef.current.checked}
							earlyUfo={earlyUfoCheckboxRef.current.checked}
							coinDemo={coinDemoCheckboxRef.current.checked}
							romDataH={romDataH}
							romDataG={romDataG}
							romDataF={romDataF}
							romDataE={romDataE}
						/>
					)
			}
		</>
	);
};

export default SiGamePanel;
