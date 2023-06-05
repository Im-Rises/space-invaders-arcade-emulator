import {InputFile} from './InputFile';
import {run} from '../../si-emu-pkg';

export const SiGameSetUp = () => {
	console.log();
	return (
		<>
			<div className={'setup-panel'}>
				<div className={'screen-mode-panel'}>
					{/* <p>Select Screen mode</p> */}
					<select id={'screen-mode'} ref={selectVersionRef}>
						<option value={'SV'}>Black and white (SV)</option>
						<option value={'TV'}>Original (TV)</option>
						<option value={'CV'}>Colored (CV)</option>
					</select>
					{/* <p>Background</p> */}
					<select id={'background'} onChange={e => {
						setIsBackgroundVisible(e.target.value === 'visible');
					}}>
						<option value={'visible'}>Visible</option>
						<option value={'hidden'}>Hidden</option>
					</select>
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
							return;
						}

						setRomsLoaded(true);
						// run(canvasId, selectVersionRef.current.value, romDataH, romDataG, romDataF, romDataE);
						run(canvasId, selectVersionRef.current.value);// Debug
					}
					}>Run
					</button>
				</div>
			</div>
		</>
	);
};
