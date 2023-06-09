import DemoVideoMode1 from '../images/demo/si-demo-1-sv.png';
import DemoVideoMode2 from '../images/demo/si-demo-2-tv.png';
import DemoVideoMode3 from '../images/demo/si-demo-3-cv.png';
import BlackBackground from '../images/background/bg_black.png';
import SIBackground1 from '../images/background/bg_invaders_1.png';
import SIBackground2 from '../images/background/bg_invaders_2.png';

const screenModeList = [{
	value: 'SV',
	label: 'Black and white (SV)',
	image: DemoVideoMode1,
}, {
	value: 'TV',
	label: 'Original (TV)',
	image: DemoVideoMode2,
}, {
	value: 'CV',
	label: 'Colored (CV)',
	image: DemoVideoMode3,
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
