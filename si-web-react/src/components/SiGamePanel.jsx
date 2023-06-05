import backgroundImage from '*.png';
import {useState} from 'react';
import React from 'react';

export const SiGamePanel = () => {
	const [isBackgroundVisible, setIsBackgroundVisible] = useState(true);
	return (
		<>
			<div className={'game-panel'}>
				<div className={'canvas-si-panel'}>
					<canvas id={canvasId} className={'canvas-si'}/>
					{
						isBackgroundVisible
							? (<img src={backgroundImage} alt={'background'}/>)
							: (<div className={'no-background'}/>)
					}
				</div>
				<div className={'control-panel'}>
					<button id={'button-left'}/>
					<button id={'button-right'}/>
					<button id={'button-up'}/>
					<button id={'button-player1-start'}/>
					<button id={'button-player2-start'}/>
					<button id={'button-coin'}/>
				</div>
			</div>
		</>
	);
};
