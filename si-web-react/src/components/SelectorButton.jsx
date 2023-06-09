import PropTypes from 'prop-types';
import {useState} from 'react';
import React from 'react';

const SelectorButton = props => {
	const defaultValue = props.elementList.find(option => option.value === props.defaultValue) || props.elementList[0];
	const [selectedOption, setSelectedOption] = useState(defaultValue);

	const handleSelectChange = event => {
		setSelectedOption(props.elementList.find(option => option.value === event.target.value));
		props.setSelectedOptionValue(selectedOption.value);
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

					props.setSelectedOptionValue(selectedOption.value);
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

					props.setSelectedOptionValue(selectedOption.value);
				}}
			>Next
			</button>
		</>
	);
};

SelectorButton.propTypes = {
	setSelectedOptionValue: PropTypes.func.isRequired,
	elementList: PropTypes.arrayOf(PropTypes.shape({
		value: PropTypes.string.isRequired,
		label: PropTypes.string.isRequired,
	})).isRequired,
	defaultValue: PropTypes.string,
};

export default SelectorButton;
