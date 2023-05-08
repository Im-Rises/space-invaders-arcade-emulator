/* global expect test */
import React from 'react';
import {render, screen} from '@testing-library/react';
import App from '../App';

test('Test render page', () => {
	render(<App/>);
	const linkElement = screen.getByTestId('canvas');
	expect(linkElement).toBeInTheDocument();
});
