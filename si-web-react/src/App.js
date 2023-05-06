import React, {useEffect, useState} from 'react';
import {InputFile} from './components/InputFile';
import './App.scss';

import init, {run} from './si-emu-pkg';

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
            {/*{*/}
            {/*romsLoaded ?*/}
            {/*(*/}
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
            // )
            // :
            // (
            <div>
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
                        console.log('romDataH', romDataH);
                        console.log('romDataG', romDataG);
                        console.log('romDataF', romDataF);
                        console.log('romDataE', romDataE);
                        return;
                    }

                    setRomsLoaded(true);
                    run(canvasId, romDataH, romDataG, romDataF, romDataE);
                }
                }>Run
                </button>
            </div>
            )
            // }
        </>
    );
};

export default App;
