import React, {useRef} from 'react';
import PropTypes from 'prop-types';

export const InputFile = ({setRomData}) => {
	const inputRef = useRef(null);

	const onFileChange = () => {
		const file = inputRef.current.files[0];
		const reader = new FileReader();
		reader.onload = () => {
			setRomData(new Uint8Array(reader.result));
		};

		reader.readAsArrayBuffer(file);
	};

	return (
		<div>
			<input type='file' ref={inputRef} onChange={onFileChange}/>
		</div>
	);
};

InputFile.propTypes = {
	setRomData: PropTypes.func.isRequired,
};
