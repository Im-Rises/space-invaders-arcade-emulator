import GameVideoMode1 from '../images/game/si-game-1-sv.png';
import GameVideoMode2 from '../images/game/si-game-2-tv.png';
import GameVideoMode3 from '../images/game/si-game-3-cv.png';
import BlackBackground from '../images/background/bg_black.png';
import SIBackground1 from '../images/background/bg_invaders_1.png';
import SIBackground2 from '../images/background/bg_invaders_2.png';

const screenModeList = [{
	value: 'SV',
	label: 'Black/white (SV)',
	image: GameVideoMode1,
}, {
	value: 'TV',
	label: 'Original (TV)',
	image: GameVideoMode2,
}, {
	value: 'CV',
	label: 'Colored (CV)',
	image: GameVideoMode3,
}];

const backgroundVersionList = [{
	value: 'hidden',
	label: 'Hidden',
	image: BlackBackground,
}, {
	value: '1',
	label: 'Version 1',
	image: SIBackground1,
}, {
	value: '2',
	label: 'Version 2',
	image: SIBackground2,
}];

export {
	screenModeList,
	backgroundVersionList,
};
