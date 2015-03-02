package com.jonalmeida.myapplication;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.text.Editable;
import android.text.TextWatcher;
import android.view.Menu;
import android.view.MenuItem;
import android.view.View;
import android.widget.ArrayAdapter;
import android.widget.Button;
import android.widget.EditText;
import android.widget.ListView;
import android.widget.TextView;

import java.util.ArrayList;
import java.util.List;


public class MainActivity extends Activity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

//        final TextView text = (TextView) findViewById(R.id.hello_world);

        Button newActivityButton = (Button) findViewById(R.id.buttonNewActivity);
        newActivityButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                createNewActivity(view);
            }
        });

        final Button changeTextButton = (Button) findViewById(R.id.buttonChangeText);
        changeTextButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                changeText();
            }
        });

        final EditText changeHelloWorld = (EditText) findViewById(R.id.editText);

        changeHelloWorld.addTextChangedListener(new TextWatcher() {
            @Override
            public void beforeTextChanged(CharSequence charSequence, int i, int i2, int i3) {

            }

            @Override
            public void onTextChanged(CharSequence charSequence, int i, int i2, int i3) {

            }

            @Override
            public void afterTextChanged(Editable editable) {
                TextView text = (TextView) findViewById(R.id.hello_world);
                text.setText(editable);
            }
        });

        populateListView();

        viewPagerButton();

    }


    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        // Inflate the menu; this adds items to the action bar if it is present.
        getMenuInflater().inflate(R.menu.menu_main, menu);
        return true;
    }

    @Override
    public boolean onOptionsItemSelected(MenuItem item) {
        // Handle action bar item clicks here. The action bar will
        // automatically handle clicks on the Home/Up button, so long
        // as you specify a parent activity in AndroidManifest.xml.
        int id = item.getItemId();

        //noinspection SimplifiableIfStatement
        if (id == R.id.action_settings) {
            return true;
        }

        return super.onOptionsItemSelected(item);
    }

    public void createNewActivity(View view) {
        Intent intent = new Intent(this, NewActivity.class);
        startActivity(intent);
    }

    public void changeText() {
        TextView text = (TextView) findViewById(R.id.hello_world);
        text.setText(R.string.changed_world);
    }

    public void populateListView() {
        ListView listView = (ListView) findViewById(R.id.listView);
//        ListView listView2 = (ListView) findViewById(R.id.listView2);
        ArrayList<String> arrayList = new ArrayList<String>();
        arrayList.add("First Array Item");
        arrayList.add("Second Array Item");
        ArrayAdapter<String> arrayAdapter = new ArrayAdapter<String>(this, android.R.layout.simple_list_item_1);
        arrayAdapter.add("First Manual Entry");
        arrayAdapter.add("Second Manual Entry");
        arrayAdapter.addAll(arrayList);

        listView.setAdapter(arrayAdapter);
//        listView2.setAdapter(arrayAdapter);
    }

    public void viewPagerButton() {
        Button button = (Button) findViewById(R.id.buttonViewPager);
        button.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {

            }
        });
    }
}
