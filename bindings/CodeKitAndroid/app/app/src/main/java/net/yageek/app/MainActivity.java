package net.yageek.app;

import androidx.appcompat.app.AppCompatActivity;

import android.graphics.Bitmap;
import android.os.Bundle;
import android.widget.ImageView;

import net.yageek.codekit.CodeKit;
import net.yageek.codekit.CodeOptions;

public class MainActivity extends AppCompatActivity {


    private ImageView imageView;
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        this.imageView = (ImageView) findViewById(R.id.imageView);
        String test = CodeKit.makeCode93("TEST93");

        CodeOptions options = new CodeOptions(200, 7, 0, 5);
        Bitmap bitmap = CodeKit.convertBitmap(test, options);

        this.imageView.setImageBitmap(bitmap);
    }
}