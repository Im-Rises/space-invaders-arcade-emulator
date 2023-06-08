import React from 'react';
import {InputFile} from './InputFile';
import {useState} from 'react';
import SiEmulator from './SiEmulator';

const SiGamePanel = () => {
	const [isRomLoaded, setRomsLoaded] = React.useState(false);
	const canvasId = 'si-canvas-id';
	const [romDataH, setRomDataH] = useState(null);
	const [romDataG, setRomDataG] = useState(null);
	const [romDataF, setRomDataF] = useState(null);
	const [romDataE, setRomDataE] = useState(null);
	const screenModeSelectRef = React.useRef(null);
	const [isBackgroundVisible, setIsBackgroundVisible] = React.useState(true);
	const oneAdditionalCheckboxRef = React.useRef(null);
	const twoAdditionalCheckboxRef = React.useRef(null);
	const earlyUfoCheckboxRef = React.useRef(null);
	const coinDemoCheckboxRef = React.useRef(null);

	return (
		<>
			{
				!isRomLoaded ? (
					<div className={'setup-panel'}>
						<div className={'screen-mode-panel'}>
							<select id={'screen-mode'} ref={screenModeSelectRef}>
								<option value={'SV'}>Black and white (SV)</option>
								<option value={'TV'}>Original (TV)</option>
								<option value={'CV'}>Colored (CV)</option>
							</select>
							<select id={'background'} onChange={e => {
								setIsBackgroundVisible(e.target.value === 'visible');
							}}>
								<option value={'visible'}>Visible</option>
								<option value={'hidden'}>Hidden</option>
							</select>
							<div>
								<label><input type={'checkbox'} ref={oneAdditionalCheckboxRef}/>One additional
                                        life</label>
								<label><input type={'checkbox'} ref={twoAdditionalCheckboxRef}/>Two additional
                                        lives</label>
								<label><input type={'checkbox'} ref={earlyUfoCheckboxRef}/>UFO at 1000 points</label>
								<label><input type={'checkbox'} ref={coinDemoCheckboxRef}/>Coin in demo</label>
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
				)
					: (
						<SiEmulator
							canvasId={canvasId}
							screenMode={screenModeSelectRef.current.value}
							isBackgroundVisible={isBackgroundVisible}
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
