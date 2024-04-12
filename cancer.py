import pandas as pd
import random as r
from sklearn.svm import SVC
from sklearn import svm
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import classification_report
from ucimlrepo import fetch_ucirepo 
from sklearn.impute import SimpleImputer

BREAST_CANCER_WISCONSIN_ID = 17
LUNG_CANCER_ID = 62

def main():  
    report = fetch_data(BREAST_CANCER_WISCONSIN_ID)
    print(report)


def fetch_data(data_id):
    data_diagnostic = fetch_ucirepo(id=data_id) 
    
    X = data_diagnostic.data.features 
    y = data_diagnostic.data.targets 

    report = predict_diagnosis(X, y, showPredictions=True)
    return report


def predict_diagnosis(X, y, showPredictions):
    imputer = SimpleImputer(strategy='mean') 
    X_imputed = imputer.fit_transform(X)
    
    X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=r.randint(1, 100))
    
    sc = StandardScaler()
    X_train_scaled = sc.fit_transform(X_train)
    X_test_scaled = sc.transform(X_test)

    clf = svm.SVC()
    clf.fit(X_train_scaled, y_train)
    pred_clf = clf.predict(X_test_scaled)

    report = classification_report(y_test, pred_clf)
    
    if showPredictions:
        for i in range(len(X_test)):
            print(f"\nSample {i + 1}: Prediction: {pred_clf[i]}, Actual: {y_test.iloc[i]}")

    return report


def init_data(data):
    data = pd.read_csv(data)


    data.dropna(inplace=True)

    data = data.drop(columns=['UID'])
    data['Diagnosis'] = data['Diagnosis'].map({'M': 1, 'B': 0})

    return data


if __name__ == "__main__": main()