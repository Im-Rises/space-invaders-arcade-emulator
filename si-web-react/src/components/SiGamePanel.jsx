import React from 'react';
import {InputFile} from './subcomponents/InputFile';
import {useState} from 'react';
import SiEmulator from './SiEmulator';
import SelectorButton from './subcomponents/SelectorButton';
import {screenModeList, backgroundVersionList} from '../constants/screen-constants';
import {ToastContainer, toast} from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import './SiGamePanel.scss';
import './subcomponents/ActionButtonStyle.scss';

const SiGamePanel = () => {
	const [isRomLoaded, setRomsLoaded] = React.useState(false);

	const [screenMode, setScreenMode] = useState('TV');
	const [backgroundVersion, setBackgroundVersion] = useState('2');

	const oneAdditionalCheckboxRef = React.useRef(null);
	const twoAdditionalCheckboxRef = React.useRef(null);
	const earlyUfoCheckboxRef = React.useRef(null);
	const coinDemoCheckboxRef = React.useRef(null);

	const [romDataH, setRomDataH] = useState(null);
	const [romDataG, setRomDataG] = useState(null);
	const [romDataF, setRomDataF] = useState(null);
	const [romDataE, setRomDataE] = useState(null);

	return (
		<>
			{
				!isRomLoaded ? (
					<>
						<ToastContainer/>
						{/* <GitHubProjectPanel link={REPO} author={AUTHOR}/> */}

						<div className={'title'}>
							<h1>Space Invaders Arcade Emulator</h1>
						</div>

						<div className={'demo-screen'}>
							<img
								src={screenModeList.find(element => element.value === screenMode).image}
								alt={'demo-game-version'} className={'img-game'}/>
							<img
								src={backgroundVersionList.find(element => element.value === backgroundVersion).image}
								alt={'demo-background-version'} className={'img-bg'}/>
						</div>

						<div className={'screen-mode-controller'}>
							<div className={'selector-button'}>
								<SelectorButton
									setSelectedOptionValue={setScreenMode}
									elementList={screenModeList}
									defaultValue={screenMode}
								/>
							</div>
							<div className={'selector-button'}>
								<SelectorButton
									setSelectedOptionValue={setBackgroundVersion}
									elementList={backgroundVersionList}
									defaultValue={backgroundVersion}
								/>
							</div>
						</div>

						<div className={'start-panel'}>
							<button className={'run-btn'} onClick={() => {
								if (!romDataH || !romDataG || !romDataF || !romDataE) {
									toast.error('Please load all ROMs');
									// return;
								}

								setRomsLoaded(true);
							}
							}>Run
							</button>
						</div>

						<div className={'rom-and-settings'}>
							<div className={'load-rom-panel'}>
								<div>
									<p>Load ROM H</p>
									<InputFile setRomData={setRomDataH}/>
								</div>
								<div>
									<p>Load ROM G</p>
									<InputFile setRomData={setRomDataG}/>
								</div>
								<div>
									<p>Load ROM F</p>
									<InputFile setRomData={setRomDataF}/>
								</div>
								<div>
									<p>Load ROM E</p>
									<InputFile setRomData={setRomDataE}/>
								</div>
							</div>
							<div className={'select-options'}>
								<div>
									<label><input type={'checkbox'} ref={oneAdditionalCheckboxRef}/>
                                            One additional life</label>
								</div>
								<div>
									<label><input type={'checkbox'} ref={twoAdditionalCheckboxRef}/>
                                            Two additional lives</label>
								</div>
								<div>
									<label><input type={'checkbox'} ref={earlyUfoCheckboxRef}/>
                                            UFO at 1000 points</label>
								</div>
								<div>
									<label><input type={'checkbox'} ref={coinDemoCheckboxRef}/>
                                            Coin in demo</label>
								</div>
							</div>
						</div>
					</>
				)
					: (
						<>
							<SiEmulator
								screenMode={screenMode}
								backgroundVersion={backgroundVersion}
								oneAdditional={oneAdditionalCheckboxRef.current.checked}
								twoAdditional={twoAdditionalCheckboxRef.current.checked}
								earlyUfo={earlyUfoCheckboxRef.current.checked}
								coinDemo={coinDemoCheckboxRef.current.checked}
								romDataH={romDataH}
								romDataG={romDataG}
								romDataF={romDataF}
								romDataE={romDataE}
							/>
							{/* <button onClick={() => { */}
							{/*	setRomsLoaded(false); */}
							{/* }}>Back */}
							{/* </button> */}
						</>
					)
			}
		</>
	);
};

export default SiGamePanel;
