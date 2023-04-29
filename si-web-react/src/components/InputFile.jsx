import React, {useRef} from 'react';

export const InputFile = props => {
	const inputRef = useRef(null);

	const onFileChange = () => {
		const file = inputRef.current.files[0];
		const reader = new FileReader();
		reader.onload = () => {
			props.setRomData(reader.result);
			props.onFileLoad();
		};

		reader.readAsArrayBuffer(file);
	};

	return (
		<div>
			<input type='file' ref={inputRef} onChange={onFileChange}/>
		</div>
	);
};
