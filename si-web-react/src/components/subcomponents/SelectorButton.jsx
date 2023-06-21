import PropTypes from 'prop-types';
import {useState} from 'react';
import React from 'react';
import './SelectorButton.scss';

const SelectorButton = props => {
	const defaultValue = props.elementList.find(option => option.value === props.defaultValue);
	const [selectedOption, setSelectedOption] = useState(defaultValue);

	const handleSelectChange = event => {
		const element = props.elementList.find(option => option.value === event.target.value);
		setSelectedOption(element);
		props.setSelectedOptionValue(element.value);
	};

	return (
		<>
			<button className={'selector-button-btn'}
				onClick={() => {
					const index = props.elementList.findIndex(option => option.value === selectedOption.value);

					let element;
					if (index === 0) {
						element = props.elementList[props.elementList.length - 1];
					} else {
						element = props.elementList[index - 1];
					}

					setSelectedOption(element);
					props.setSelectedOptionValue(element.value);
				}}
			>←
			</button>
			<select className={'selector-button-select'} value={selectedOption.value} onChange={handleSelectChange}>
				{props.elementList.map((option, index) => (
					<option key={index} value={option.value}>
						{props.elementList[index].label}
					</option>
				))}
			</select>
			<button className={'selector-button-btn'}
				onClick={() => {
					const index = props.elementList.findIndex(option => option.value === selectedOption.value);

					let element;
					if (index === props.elementList.length - 1) {
						element = props.elementList[0];
					} else {
						element = props.elementList[index + 1];
					}

					setSelectedOption(element);
					props.setSelectedOptionValue(element.value);
					console.log(element);
				}}
			>→
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
	defaultValue: PropTypes.string.isRequired,
};

export default SelectorButton;
