function [n, opt, greedy, equal, random] = get_data(filename)

    delimiter = ',';
    startRow = 3;

    formatSpec = '%f%f%f%f%f %[^\n\r]';
    fileID = fopen(filename,'r');

    dataArray = textscan(fileID, formatSpec, 'Delimiter', delimiter, 'HeaderLines' ,startRow-1, 'ReturnOnError', false);
    
    fclose(fileID);

    n = dataArray{:, 1};
    opt = dataArray{:, 2};
    greedy = dataArray{:, 3};
    equal = dataArray{:, 4};
    random = dataArray{:, 5};

    clearvars filename delimiter startRow formatSpec fileID dataArray ans;