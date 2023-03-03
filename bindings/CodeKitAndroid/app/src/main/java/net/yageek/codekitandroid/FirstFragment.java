package net.yageek.codekitandroid;

import android.graphics.Bitmap;
import android.os.Bundle;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;

import androidx.annotation.NonNull;
import androidx.fragment.app.Fragment;
import androidx.navigation.fragment.NavHostFragment;

import net.yageek.codekit.CodeDescriptor;
import net.yageek.codekit.CodeKit;
import net.yageek.codekit.CodeOptions;
import net.yageek.codekitandroid.databinding.FragmentFirstBinding;

public class FirstFragment extends Fragment {

    private FragmentFirstBinding binding;

    @Override
    public View onCreateView(
            LayoutInflater inflater, ViewGroup container,
            Bundle savedInstanceState
    ) {

        binding = FragmentFirstBinding.inflate(inflater, container, false);
        return binding.getRoot();

    }

    public void onViewCreated(@NonNull View view, Bundle savedInstanceState) {
        super.onViewCreated(view, savedInstanceState);

        CodeOptions options = new CodeOptions(7, 5, 0);
        CodeDescriptor value = CodeKit.makeEAN8("6583-3254", options);
        Bitmap bitmap = CodeKit.convertBitmap(value);

        // Scaling the bitmap

        int width = 600;
        int height = 400;
       Bitmap scaled = Bitmap.createScaledBitmap(bitmap, width, height, true);
        binding.imageView.setImageBitmap(scaled);
    }


    @Override
    public void onDestroyView() {
        super.onDestroyView();
        binding = null;
    }

}