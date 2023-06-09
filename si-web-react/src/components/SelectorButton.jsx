import PropTypes from 'prop-types';
import {useState} from 'react';
import React from 'react';

const SelectorButton = props => {
	const [selectedOption, setSelectedOption] = useState(props.defaultValue || props.elementList[0]);

	const handleSelectChange = event => {
		setSelectedOption(event.target.value);
	};

	return (
		<>
			<button
				onClick={() => {
					const index = props.elementList.findIndex(option => option.value === selectedOption.value);

					if (index === 0) {
						setSelectedOption(props.elementList[props.elementList.length - 1]);
					} else {
						setSelectedOption(props.elementList[index - 1]);
					}
				}}
			>Previous
			</button>
			<select value={selectedOption.value} onChange={handleSelectChange}>
				{props.elementList.map((option, index) => (
					<option key={index} value={option.value}>
						{props.elementList[index].label}
					</option>
				))}
			</select>
			<button
				onClick={() => {
					const index = props.elementList.findIndex(option => option.value === selectedOption.value);

					if (index === props.elementList.length - 1) {
						setSelectedOption(props.elementList[0]);
					} else {
						setSelectedOption(props.elementList[index + 1]);
					}
				}}
			>Next
			</button>
		</>
	);
};

SelectorButton.propTypes = {
	elementList: PropTypes.arrayOf(PropTypes.shape({
		value: PropTypes.string.isRequired,
		label: PropTypes.string.isRequired,
	})).isRequired,
	defaultValue: PropTypes.string,
};

export default SelectorButton;
