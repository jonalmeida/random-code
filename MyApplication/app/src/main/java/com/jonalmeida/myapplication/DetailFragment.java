package com.jonalmeida.myapplication;

import android.app.Activity;
import android.app.Fragment;
import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;

public class DetailFragment extends Fragment {

    OnMessageReceivedFromTop mListener;

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container,
                             Bundle savedInstanceState) {
        View view =  inflater.inflate(R.layout.fragment_rssitem_detail, container, false);

        createButton(view);

        return view;
    }
    
    @Override
    public void onAttach(Activity activity) {
        super.onAttach(activity);
        mListener = (OnMessageReceivedFromTop) activity;
    }

    public void setText(String item) {
        TextView tv = (TextView) getView().findViewById(R.id.detailsText);
        tv.setText(item);
    }

    public interface OnMessageReceivedFromTop {
        public void onMessageReceivedFromTop(String text, int id);
    }

    public void createButton(View view) {
        Button myButton = (Button) view.findViewById(R.id.fragmentButton);
        myButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                EditText editText = (EditText) getView().findViewById(R.id.fragmentEditText);
                mListener.onMessageReceivedFromTop(editText.getText().toString(), getId());
            }
        });
    }
}
